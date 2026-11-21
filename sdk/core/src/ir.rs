//! The DSL interpreter: it executes the canonical IR that the Vane, Cadence
//! and Glyph front ends lower to (docs/dsl-ir.md). A program's meaning is
//! its IR's meaning, and this is the only implementation of it: the browser,
//! the CLI and the hosted arena all run this code, compiled to WebAssembly.
//!
//! The interpreter knows no DSL. It decodes and checks an IR module, keeps a
//! robot's cells between decisions, and runs one decision on the turn's
//! words record and contact snapshot. Operations are counted by CHARGE
//! instructions alone, so how this file executes the rest never changes a
//! robot's counted cost.
//!
//! Values and arithmetic are JavaScript's, exactly as docs/dsl-ir.md
//! defines them; transcendental functions come from libm (maths.rs), so
//! every host computes the same bits.

use crate::maths::{atan2, cos, max, min, sin, sqrt, PI};

pub const VERSION: u16 = 1;
pub const MAGIC: &[u8; 4] = b"CYIR";
/// Words in the cy_dsl_words_v1 record, one f64 each.
pub const WORDS: usize = core::mem::size_of::<crate::generated::DslWordsV1>() / 8;
/// f64 fields in each contact row: slot, source, range, bearing, elevation,
/// bearing error, range error, x, y, z.
pub const CONTACT_FIELDS: usize = 10;
/// The decision's controls, in IR field order; new fields go on the end.
pub const FIELDS: [&str; 27] = [
    "throttle", "turn", "pitchTurn", "thrust", "strafe", "lift", "yawTorque", "pitchTorque", "rollTorque", "brake",
    "aim", "aimPitch", "scanHeading", "scanPitch", "scanArc", "fire", "power", "botStatusText", "botStatusValue",
    "launch", "launchHeading", "launchPitch", "launchParam", "launchBrief1", "launchBrief2", "launchBrief3", "launchBrief4",
];
const BRAKE: u32 = 9;
const FIRE: u32 = 15;
const TEXT: u32 = 17;
const VALUE: u32 = 18;
const LAUNCH: u32 = 19;
/// CLAIM targets at or above this are cells.
pub const CELL_TARGET: u32 = 32;
pub const NONE: u32 = u32::MAX;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Op {
    Charge = 0,
    Push,
    Load,
    Get,
    Set,
    Local,
    Bind,
    Call,
    Depth,
    Ret,
    Done,
    Jump,
    JumpUnless,
    JumpIf,
    And,
    Or,
    Bool,
    Not,
    Neg,
    Math,
    Cmp,
    Logic,
    Dup,
    Drop,
    Swap,
    Over,
    Contact,
    Out,
    Claim,
    Fail,
    Table,
    After,
    Hold,
}
const OPS: u8 = Op::Hold as u8 + 1;

impl Op {
    fn from(byte: u8) -> Option<Op> {
        // SAFETY: Op is repr(u8) with contiguous discriminants 0..OPS.
        (byte < OPS).then(|| unsafe { core::mem::transmute::<u8, Op>(byte) })
    }
}

/// MATH functions, operand a & 0xff.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Func {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Min,
    Max,
    Abs,
    Sqrt,
    Clamp,
    Clamp1,
    Sin,
    Cos,
    Wave,
    Sweep,
    Angle,
    Atan2,
}
const FN_NAMES: [&str; 17] = ["+", "-", "*", "/", "%", "min", "max", "abs", "sqrt", "clamp", "clamp1", "sin", "cos", "wave", "sweep", "angle", "atan2"];

impl Func {
    fn from(byte: u32) -> Option<Func> {
        use Func::*;
        [Add, Sub, Mul, Div, Rem, Min, Max, Abs, Sqrt, Clamp, Clamp1, Sin, Cos, Wave, Sweep, Angle, Atan2].get(byte as usize).copied()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Instr {
    pub op: Op,
    pub a: u32,
    pub b: u32,
    pub pos: u32,
}

/// A value: JavaScript's number, boolean, string (an index into the
/// module's interned strings, so equal indices are equal strings) or null.
#[derive(Clone, Copy, Debug)]
pub enum Value {
    Num(f64),
    Bool(bool),
    Str(u32),
    Null,
}

#[derive(Clone, Debug)]
pub enum Source {
    /// A number word: the record's f64.
    Number(u32),
    /// A boolean word: the f64 is not 0.
    Flag(u32),
    /// An enum word: its names by index (NONE reads null).
    Enum(u32, Vec<u32>),
    Cell(u32),
    /// A strategist parameter: its value, else 0.
    Param,
    ContactCount,
}

#[derive(Clone, Debug)]
pub struct Slot {
    pub name: u32,
    pub sources: Vec<Source>,
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub name: u32,
    pub slot: u32,
    pub init: Option<Value>,
}

#[derive(Clone, Debug)]
pub struct Module {
    pub language: u8,
    pub source: String,
    pub stack_limit: u32,
    pub depth_limit: u32,
    pub strings: Vec<(String, f64)>,
    pub constants: Vec<Value>,
    pub positions: Vec<(u32, u32, u32)>,
    pub slots: Vec<Slot>,
    pub cells: Vec<Cell>,
    pub tables: Vec<Vec<u32>>,
    pub code: Vec<Instr>,
}

struct Reader<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl Reader<'_> {
    fn take(&mut self, n: usize) -> Result<&[u8], String> {
        let end = self.at.checked_add(n).filter(|end| *end <= self.bytes.len()).ok_or("the IR ends early")?;
        let slice = &self.bytes[self.at..end];
        self.at = end;
        Ok(slice)
    }
    fn u8(&mut self) -> Result<u8, String> {
        Ok(self.take(1)?[0])
    }
    fn u32(&mut self) -> Result<u32, String> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap_or_default()))
    }
    fn f64(&mut self) -> Result<f64, String> {
        Ok(f64::from_le_bytes(self.take(8)?.try_into().unwrap_or_default()))
    }
    fn count(&mut self) -> Result<usize, String> {
        let n = self.u32()? as usize;
        // Every entry takes at least a byte, so a count cannot exceed what is left.
        if n > self.bytes.len() - self.at { Err("an IR count exceeds the module".into()) } else { Ok(n) }
    }
    fn string(&mut self) -> Result<String, String> {
        let n = self.count()?;
        String::from_utf8(self.take(n)?.to_vec()).map_err(|_| "an IR string is not UTF-8".into())
    }
}

impl Module {
    /// Decodes and checks an IR module: every index in range, so a decision
    /// can fail only as docs/dsl-ir.md says it may.
    pub fn decode(bytes: &[u8]) -> Result<Module, String> {
        let mut r = Reader { bytes, at: 0 };
        if r.take(4)? != MAGIC {
            return Err("not a Clunkyard DSL IR module".into());
        }
        let version = u16::from_le_bytes(r.take(2)?.try_into().unwrap_or_default());
        if version != VERSION {
            return Err(format!("IR version {version} is not {VERSION}"));
        }
        let language = r.u8()?;
        let source = r.string()?;
        let stack_limit = r.u32()?;
        let depth_limit = r.u32()?;
        let mut strings = Vec::new();
        for _ in 0..r.count()? {
            strings.push((r.string()?, r.f64()?));
        }
        let ns = strings.len() as u32;
        let value = |r: &mut Reader| -> Result<Value, String> {
            Ok(match r.u8()? {
                0 => Value::Num(r.f64()?),
                1 => Value::Bool(r.u8()? != 0),
                2 => {
                    let s = r.u32()?;
                    if s >= ns {
                        return Err("an IR string index is out of range".into());
                    }
                    Value::Str(s)
                }
                3 => Value::Null,
                _ => return Err("an IR value has an unknown tag".into()),
            })
        };
        let mut constants = Vec::new();
        for _ in 0..r.count()? {
            constants.push(value(&mut r)?);
        }
        let mut positions = Vec::new();
        for _ in 0..r.count()? {
            positions.push((r.u32()?, r.u32()?, r.u32()?));
        }
        let mut slots = Vec::new();
        for _ in 0..r.count()? {
            let name = r.u32()?;
            let mut sources = Vec::new();
            for _ in 0..r.count()? {
                sources.push(match r.u8()? {
                    0 => Source::Number(r.u32()?),
                    1 => Source::Flag(r.u32()?),
                    2 => {
                        let word = r.u32()?;
                        let mut names = Vec::new();
                        for _ in 0..r.count()? {
                            names.push(r.u32()?);
                        }
                        Source::Enum(word, names)
                    }
                    3 => Source::Cell(r.u32()?),
                    4 => Source::Param,
                    5 => Source::ContactCount,
                    _ => return Err("an IR slot source has an unknown kind".into()),
                });
            }
            slots.push(Slot { name, sources });
        }
        let mut cells = Vec::new();
        for _ in 0..r.count()? {
            let name = r.u32()?;
            let slot = r.u32()?;
            let init = if r.u8()? != 0 { Some(value(&mut r)?) } else { None };
            cells.push(Cell { name, slot, init });
        }
        let mut tables = Vec::new();
        for _ in 0..r.count()? {
            let mut table = Vec::new();
            for _ in 0..r.count()? {
                table.push(r.u32()?);
            }
            tables.push(table);
        }
        let mut code = Vec::new();
        for _ in 0..r.count()? {
            let op = Op::from(r.u8()?).ok_or("an IR instruction has an unknown operation")?;
            code.push(Instr { op, a: r.u32()?, b: r.u32()?, pos: r.u32()? });
        }
        if r.at != bytes.len() {
            return Err("the IR has trailing bytes".into());
        }
        let m = Module { language, source, stack_limit, depth_limit, strings, constants, positions, slots, cells, tables, code };
        m.check()?;
        Ok(m)
    }

    fn check(&self) -> Result<(), String> {
        let (ns, nc, nslots, ncells, npc) = (self.strings.len() as u32, self.constants.len() as u32, self.slots.len() as u32, self.cells.len() as u32, self.code.len() as u32);
        let bad = |what: &str| Err(format!("an IR {what} is out of range"));
        if self.code.is_empty() {
            return bad("program");
        }
        for (_, _, text) in &self.positions {
            if *text >= ns {
                return bad("position");
            }
        }
        for slot in &self.slots {
            if slot.name >= ns {
                return bad("slot name");
            }
            for source in &slot.sources {
                match source {
                    Source::Number(w) | Source::Flag(w) if *w as usize >= WORDS => return bad("word"),
                    Source::Enum(w, names) if *w as usize >= WORDS || names.iter().any(|n| *n != NONE && *n >= ns) => return bad("word"),
                    Source::Cell(c) if *c >= ncells => return bad("cell"),
                    _ => {}
                }
            }
        }
        for cell in &self.cells {
            if cell.name >= ns || (cell.slot != NONE && cell.slot >= nslots) {
                return bad("cell");
            }
        }
        for (pc, i) in self.code.iter().enumerate() {
            // Jumps only go forward, so a decision ends: only calls go back,
            // and the depth limit bounds them.
            let pc = pc as u32;
            let forward = match i.op {
                Op::Jump | Op::JumpUnless | Op::JumpIf | Op::And | Op::Or => i.a > pc,
                Op::Claim => i.b > pc,
                Op::Table => self.tables.get(i.b as usize).is_some_and(|table| table.iter().all(|target| *target > pc)),
                _ => true,
            };
            if !forward {
                return Err("an IR jump goes backward".into());
            }
        }
        for table in &self.tables {
            if table.iter().any(|pc| *pc >= npc) {
                return bad("jump table");
            }
        }
        for i in &self.code {
            if i.pos != NONE && i.pos as usize >= self.positions.len() {
                return bad("instruction position");
            }
            let ok = match i.op {
                Op::Push => i.a < nc,
                Op::Load => i.a < nslots,
                Op::Get | Op::Set | Op::After | Op::Hold => i.a < ncells,
                Op::Call | Op::Jump | Op::JumpUnless | Op::JumpIf | Op::And | Op::Or => i.a < npc,
                Op::Claim => i.b < npc && (i.a < FIELDS.len() as u32 || (i.a >= CELL_TARGET && i.a - CELL_TARGET < ncells)),
                Op::Math => match Func::from(i.a & 0xff) {
                    Some(Func::Wave) => (i.b & 0xffff) < nslots,
                    Some(Func::Sweep) => (i.b & 0xffff) < nslots && (i.b >> 16) < nslots,
                    Some(Func::Min | Func::Max) => i.b <= 4096,
                    Some(Func::Clamp) => i.b == 3,
                    Some(Func::Abs | Func::Sqrt | Func::Clamp1 | Func::Sin | Func::Cos) => i.b == 1,
                    Some(_) => i.b == 2,
                    None => false,
                },
                Op::Cmp => i.a < 6,
                Op::Logic => i.a < 2,
                Op::Contact => (i.a as usize) < CONTACT_FIELDS,
                Op::Out => (i.a as usize) < FIELDS.len(),
                Op::Fail => i.a < ns,
                Op::Table => i.a < ncells && (i.b as usize) < self.tables.len(),
                _ => true,
            };
            if !ok {
                return bad("operand");
            }
        }
        Ok(())
    }
}

/// A runtime error: where, and what.
#[derive(Debug)]
pub struct Fault {
    pub pos: u32,
    pub message: String,
}

/// What one decision wrote, by IR field.
#[derive(Default, Debug)]
pub struct Output {
    pub numbers: [Option<f64>; FIELDS.len()],
    pub fire: Option<bool>,
    pub launch: Option<bool>,
    pub text: Option<String>,
    pub ops: u32,
}

/// A robot's program and the state it keeps between decisions.
#[derive(Clone)]
pub struct Machine {
    pub module: std::rc::Rc<Module>,
    pub cells: Vec<Option<Value>>,
}

/// One decision's inputs.
pub struct Turn<'a> {
    pub words: &'a [f64],
    /// None when the program does not read contacts.
    pub contacts: Option<&'a [f64]>,
    /// A value for every slot; only Param slots read theirs.
    pub params: &'a [f64],
    pub budget: u32,
}

impl Machine {
    pub fn new(module: Module) -> Machine {
        let cells = module.cells.iter().map(|cell| cell.init).collect();
        Machine { module: std::rc::Rc::new(module), cells }
    }

    /// Runs one decision. Cells keep what it wrote even when it fails.
    pub fn decide(&mut self, turn: &Turn) -> Result<Output, (Fault, u32)> {
        let mut run = Run { m: &self.module, cells: &mut self.cells, turn, slots: Vec::new(), stack: Vec::new(), locals: Vec::new(), frames: Vec::new(), left: turn.budget, out: Output::default(), owned: Vec::new() };
        run.init();
        match run.go() {
            Ok(()) => {
                run.out.ops = turn.budget - run.left;
                Ok(run.out)
            }
            Err(fault) => Err((fault, turn.budget - run.left)),
        }
    }

    pub fn position(&self, pos: u32) -> (u32, u32) {
        self.module.positions.get(pos as usize).map_or((1, 1), |p| (p.0, p.1))
    }
}

struct Run<'a> {
    m: &'a Module,
    cells: &'a mut Vec<Option<Value>>,
    turn: &'a Turn<'a>,
    slots: Vec<Option<Value>>,
    stack: Vec<Value>,
    locals: Vec<Value>,
    /// (return pc, locals base) for each Glyph word call.
    frames: Vec<(usize, usize)>,
    left: u32,
    out: Output,
    owned: Vec<u32>,
}

/// JavaScript's Boolean(value).
fn truthy(m: &Module, v: Value) -> bool {
    match v {
        Value::Num(n) => !(n == 0.0 || n.is_nan()),
        Value::Bool(b) => b,
        Value::Str(s) => !m.strings[s as usize].0.is_empty(),
        Value::Null => false,
    }
}

/// JavaScript's Number(value); a string's is computed by the front end.
fn to_number(m: &Module, v: Value) -> f64 {
    match v {
        Value::Num(n) => n,
        Value::Bool(b) => f64::from(u8::from(b)),
        Value::Str(s) => m.strings[s as usize].1,
        Value::Null => 0.0,
    }
}

/// JavaScript's ===.
fn strict_equal(a: Value, b: Value) -> bool {
    match (a, b) {
        (Value::Num(x), Value::Num(y)) => x == y,
        (Value::Bool(x), Value::Bool(y)) => x == y,
        (Value::Str(x), Value::Str(y)) => x == y,
        (Value::Null, Value::Null) => true,
        _ => false,
    }
}

/// JavaScript's String(value).
fn to_text(m: &Module, v: Value) -> String {
    match v {
        Value::Num(n) => number_text(n),
        Value::Bool(b) => if b { "true" } else { "false" }.into(),
        Value::Str(s) => m.strings[s as usize].0.clone(),
        Value::Null => "null".into(),
    }
}

/// ECMAScript's Number::toString(x), radix 10.
pub fn number_text(x: f64) -> String {
    if x.is_nan() {
        return "NaN".into();
    }
    if x == 0.0 {
        return "0".into();
    }
    if x.is_infinite() {
        return if x > 0.0 { "Infinity" } else { "-Infinity" }.into();
    }
    if x < 0.0 {
        return format!("-{}", number_text(-x));
    }
    // Rust's {:e} gives the shortest digits that round-trip, as ECMAScript asks.
    let e = format!("{x:e}");
    let (mantissa, exponent) = e.split_once('e').unwrap_or((&e, "0"));
    let digits: String = mantissa.chars().filter(|c| *c != '.').collect();
    let k = digits.len() as i32;
    let n = exponent.parse::<i32>().unwrap_or(0) + 1;
    if k <= n && n <= 21 {
        format!("{digits}{}", "0".repeat((n - k) as usize))
    } else if 0 < n && n <= 21 {
        format!("{}.{}", &digits[..n as usize], &digits[n as usize..])
    } else if -6 < n && n <= 0 {
        format!("0.{}{digits}", "0".repeat((-n) as usize))
    } else {
        let sign = if n - 1 < 0 { '-' } else { '+' };
        let rest = if k > 1 { format!(".{}", &digits[1..]) } else { String::new() };
        format!("{}{rest}e{sign}{}", &digits[..1], (n - 1).abs())
    }
}

/// sin and cos of degrees, exactly 0 where they cross zero.
fn sin_degrees(d: f64) -> f64 {
    if d % 180.0 == 0.0 { 0.0 } else { sin(d * PI / 180.0) }
}
fn cos_degrees(d: f64) -> f64 {
    if (d - 90.0) % 180.0 == 0.0 { 0.0 } else { cos(d * PI / 180.0) }
}
fn angle_degrees(target: f64, current: f64) -> f64 {
    ((target - current + 180.0) % 360.0 + 360.0) % 360.0 - 180.0
}

const PURPOSE_ARGUMENT: u32 = 1;

impl Run<'_> {
    fn init(&mut self) {
        let m = self.m;
        let words = self.turn.words;
        self.slots = m
            .slots
            .iter()
            .enumerate()
            .map(|(i, slot)| {
                slot.sources.iter().find_map(|source| match source {
                    Source::Number(w) => Some(Value::Num(words[*w as usize])),
                    Source::Flag(w) => Some(Value::Bool(words[*w as usize] != 0.0)),
                    Source::Enum(w, names) => {
                        let at = words[*w as usize];
                        let name = if at >= 0.0 && at.fract() == 0.0 { names.get(at as usize).copied().unwrap_or(NONE) } else { NONE };
                        Some(if name == NONE { Value::Null } else { Value::Str(name) })
                    }
                    Source::Cell(c) => self.cells[*c as usize],
                    Source::Param => Some(Value::Num(self.turn.params.get(i).copied().unwrap_or(0.0))),
                    Source::ContactCount => Some(Value::Num(self.turn.contacts.map_or(0, |c| c.len() / CONTACT_FIELDS) as f64)),
                })
            })
            .collect();
    }

    fn fault(&self, pos: u32, message: impl Into<String>) -> Fault {
        Fault { pos, message: message.into() }
    }

    fn text(&self, pos: u32) -> &str {
        self.m.positions.get(pos as usize).map_or("", |p| self.m.strings[p.2 as usize].0.as_str())
    }

    fn push(&mut self, v: Value, pos: u32) -> Result<(), Fault> {
        if self.stack.len() >= self.m.stack_limit as usize {
            return Err(self.fault(pos, format!("stack exceeds {} values", self.m.stack_limit)));
        }
        self.stack.push(v);
        Ok(())
    }

    fn pop(&mut self, pos: u32, label: &str) -> Result<Value, Fault> {
        match self.stack.pop() {
            Some(v) => Ok(v),
            None => Err(self.fault(pos, format!("stack underflow: {} needs {label}", self.text(pos)))),
        }
    }

    /// JavaScript's Number(value), which must be finite.
    fn finite(&self, v: Value, pos: u32, purpose: &str) -> Result<f64, Fault> {
        let n = to_number(self.m, v);
        if n.is_finite() { Ok(n) } else { Err(self.fault(pos, format!("{purpose} did not produce a finite number"))) }
    }

    fn numeric(&mut self, pos: u32, label: &str, purpose: &str) -> Result<f64, Fault> {
        let v = self.pop(pos, label)?;
        self.finite(v, pos, purpose)
    }

    fn set_cell(&mut self, cell: u32, v: Value) {
        self.cells[cell as usize] = Some(v);
        let slot = self.m.cells[cell as usize].slot;
        if slot != NONE {
            self.slots[slot as usize] = Some(v);
        }
    }

    fn slot_number(&self, slot: u32) -> f64 {
        self.slots[slot as usize].map_or(f64::NAN, |v| to_number(self.m, v))
    }

    fn go(&mut self) -> Result<(), Fault> {
        let m = self.m;
        let mut pc = 0usize;
        loop {
            let Some(&i) = m.code.get(pc) else { return Ok(()) };
            pc += 1;
            let pos = i.pos;
            match i.op {
                Op::Charge => {
                    if self.left == 0 {
                        return Err(self.fault(pos, format!("execution budget exceeded ({} operations)", self.turn.budget)));
                    }
                    self.left -= 1;
                }
                Op::Push => self.push(m.constants[i.a as usize], pos)?,
                Op::Load => match self.slots[i.a as usize] {
                    Some(v) => self.push(v, pos)?,
                    None => return Err(self.fault(pos, format!("unknown value {}", m.strings[m.slots[i.a as usize].name as usize].0))),
                },
                Op::Get => {
                    let v = self.cells[i.a as usize].unwrap_or(Value::Null);
                    self.push(v, pos)?;
                }
                Op::Set => {
                    let v = self.pop(pos, "a value to store")?;
                    if matches!(v, Value::Num(_)) {
                        self.finite(v, pos, "stored value")?;
                    }
                    self.set_cell(i.a, v);
                    if i.b & 1 != 0 {
                        self.owned.push(CELL_TARGET + i.a);
                    }
                }
                Op::Local => {
                    let base = self.frames.last().map_or(0, |f| f.1);
                    let v = self.locals[base + i.a as usize];
                    self.push(v, pos)?;
                }
                Op::Bind => {
                    let v = self.pop(pos, "a value for the local")?;
                    self.locals.push(v);
                }
                Op::Call => {
                    // depth reports the limit where the program says; this
                    // stops a module that has no depth instruction.
                    if self.frames.len() > m.depth_limit as usize {
                        return Err(self.fault(pos, format!("call depth exceeds {}", m.depth_limit)));
                    }
                    self.frames.push((pc, self.locals.len()));
                    pc = i.a as usize;
                }
                Op::Depth => {
                    if self.frames.len() > m.depth_limit as usize {
                        return Err(self.fault(pos, format!("call depth exceeds {}", m.depth_limit)));
                    }
                }
                Op::Ret => match self.frames.pop() {
                    Some((back, base)) => {
                        self.locals.truncate(base);
                        pc = back;
                    }
                    None => return Ok(()),
                },
                Op::Done => return Ok(()),
                Op::Jump => pc = i.a as usize,
                Op::JumpUnless | Op::JumpIf => {
                    let v = self.pop(pos, "a condition")?;
                    if truthy(m, v) == (i.op == Op::JumpIf) {
                        pc = i.a as usize;
                    }
                }
                Op::And | Op::Or => {
                    let v = self.pop(pos, "value")?;
                    let held = truthy(m, v);
                    if held == (i.op == Op::Or) {
                        self.push(Value::Bool(held), pos)?;
                        pc = i.a as usize;
                    }
                }
                Op::Bool => {
                    let v = self.pop(pos, "value")?;
                    self.push(Value::Bool(truthy(m, v)), pos)?;
                }
                Op::Not => {
                    let v = self.pop(pos, "value")?;
                    self.push(Value::Bool(!truthy(m, v)), pos)?;
                }
                Op::Neg => {
                    let n = self.numeric(pos, "number", "numeric expression")?;
                    self.push(Value::Num(-n), pos)?;
                }
                Op::Math => self.math(i)?,
                Op::Cmp => {
                    let b = self.pop(pos, "value")?;
                    let a = self.pop(pos, "value")?;
                    let result = match i.a {
                        0 => strict_equal(a, b),
                        1 => !strict_equal(a, b),
                        op => {
                            let ordering = match (a, b) {
                                (Value::Str(x), Value::Str(y)) => Some(m.strings[x as usize].0.encode_utf16().cmp(m.strings[y as usize].0.encode_utf16())),
                                _ => to_number(m, a).partial_cmp(&to_number(m, b)),
                            };
                            match (op, ordering) {
                                (_, None) => false,
                                (2, Some(o)) => o.is_lt(),
                                (3, Some(o)) => o.is_le(),
                                (4, Some(o)) => o.is_gt(),
                                (_, Some(o)) => o.is_ge(),
                            }
                        }
                    };
                    self.push(Value::Bool(result), pos)?;
                }
                Op::Logic => {
                    let b = self.pop(pos, "value")?;
                    let a = self.pop(pos, "value")?;
                    let (a, b) = (truthy(m, a), truthy(m, b));
                    self.push(Value::Bool(if i.a == 0 { a && b } else { a || b }), pos)?;
                }
                Op::Dup => {
                    let v = self.pop(pos, "value")?;
                    self.push(v, pos)?;
                    self.push(v, pos)?;
                }
                Op::Drop => {
                    self.pop(pos, "value")?;
                }
                Op::Swap => {
                    let b = self.pop(pos, "value")?;
                    let a = self.pop(pos, "value")?;
                    self.push(b, pos)?;
                    self.push(a, pos)?;
                }
                Op::Over => {
                    if self.stack.len() < 2 {
                        return Err(self.fault(pos, "stack underflow: over needs two values"));
                    }
                    let v = self.stack[self.stack.len() - 2];
                    self.push(v, pos)?;
                }
                Op::Contact => {
                    let v = self.pop(pos, "a contact index")?;
                    let at = self.finite(v, pos, "contact index")?;
                    let contacts = self.turn.contacts.unwrap_or(&[]);
                    let count = contacts.len() / CONTACT_FIELDS;
                    if at.fract() != 0.0 || at.abs() > 9007199254740991.0 || at < 0.0 || at >= count as f64 {
                        return Err(self.fault(pos, format!("contact index {} is out of range", number_text(at))));
                    }
                    self.push(Value::Num(contacts[at as usize * CONTACT_FIELDS + i.a as usize]), pos)?;
                }
                Op::Out => self.out(i)?,
                Op::Claim => {
                    if self.owned.contains(&i.a) {
                        pc = i.b as usize;
                    }
                }
                Op::Fail => return Err(self.fault(pos, m.strings[i.a as usize].0.clone())),
                Op::Table => {
                    let at = self.cells[i.a as usize].map_or(f64::NAN, |v| to_number(m, v));
                    let table = &m.tables[i.b as usize];
                    match (at >= 0.0 && at.fract() == 0.0).then(|| table.get(at as usize)).flatten() {
                        Some(target) => pc = *target as usize,
                        None => return Ok(()),
                    }
                }
                Op::After => {
                    let v = self.pop(pos, "value")?;
                    let duration = to_number(m, v);
                    if !duration.is_finite() || duration < 0.0 {
                        return Err(self.fault(pos, "after duration must be a non-negative number"));
                    }
                    let ticks = self.cells[i.a as usize].map_or(0.0, |v| to_number(m, v));
                    self.push(Value::Bool(ticks >= duration), pos)?;
                }
                Op::Hold => {
                    let v = self.pop(pos, "value")?;
                    let now = truthy(m, v);
                    let count = if now { self.cells[i.a as usize].map_or(0.0, |v| to_number(m, v)) + 1.0 } else { 0.0 };
                    self.cells[i.a as usize] = Some(Value::Num(count));
                    self.push(Value::Bool(now && count >= f64::from(i.b)), pos)?;
                }
            }
        }
    }

    fn math(&mut self, i: Instr) -> Result<(), Fault> {
        let pos = i.pos;
        let f = Func::from(i.a & 0xff).unwrap_or(Func::Add);
        let argc = match f {
            Func::Wave => 2,
            Func::Sweep => 1,
            _ => i.b as usize,
        };
        let purpose = if (i.a >> 8) & 1 == PURPOSE_ARGUMENT { format!("{} argument", FN_NAMES[f as usize]) } else { "numeric expression".to_string() };
        let label = match (f, argc) {
            (Func::Clamp, _) => "value, low, and high",
            (_, 1) => "number",
            _ => "two numbers",
        };
        let mut args = [0.0f64; 3];
        let mut many = Vec::new();
        for k in (0..argc).rev() {
            let n = self.numeric(pos, label, &purpose)?;
            if argc <= 3 { args[k] = n } else { many.push(n) }
        }
        if argc > 3 {
            many.reverse();
        }
        let a = |k: usize| args[k];
        let result = match f {
            Func::Add => a(0) + a(1),
            Func::Sub => a(0) - a(1),
            Func::Mul => a(0) * a(1),
            Func::Div | Func::Rem => {
                if a(1) == 0.0 {
                    return Err(self.fault(pos, if f == Func::Div { "division by zero" } else { "remainder by zero" }));
                }
                if f == Func::Div { a(0) / a(1) } else { a(0) % a(1) }
            }
            Func::Min | Func::Max => {
                let values: &[f64] = if argc > 3 { &many } else { &args[..argc] };
                let pick = if f == Func::Min { min } else { max };
                values.iter().fold(if f == Func::Min { f64::INFINITY } else { f64::NEG_INFINITY }, |acc, v| pick(acc, *v))
            }
            Func::Abs => a(0).abs(),
            Func::Sqrt => {
                if a(0) < 0.0 {
                    return Err(self.fault(pos, "square root of a negative number"));
                }
                sqrt(a(0))
            }
            Func::Clamp => max(a(1), min(a(2), a(0))),
            Func::Clamp1 => max(-1.0, min(1.0, a(0))),
            Func::Sin => sin_degrees(a(0)),
            Func::Cos => cos_degrees(a(0)),
            Func::Wave => {
                if a(0) == 0.0 {
                    return Err(self.fault(pos, "wave period cannot be zero"));
                }
                sin_degrees(self.slot_number(i.b & 0xffff) * 360.0 / a(0)) * a(1)
            }
            Func::Sweep => ((self.slot_number(i.b >> 16) + self.slot_number(i.b & 0xffff) * a(0)) % 360.0 + 360.0) % 360.0,
            Func::Angle => angle_degrees(a(0), a(1)),
            Func::Atan2 => atan2(a(0), a(1)) * 180.0 / PI,
        };
        self.push(Value::Num(result), pos)
    }

    fn out(&mut self, i: Instr) -> Result<(), Fault> {
        let pos = i.pos;
        let field = i.a;
        let labelled = i.b & 2 != 0;
        let purpose = if labelled { FIELDS[field as usize] } else { "numeric expression" };
        let label = match field {
            BRAKE => "a brake strength",
            FIRE => "a firing condition",
            LAUNCH => "a launch condition",
            TEXT => "status text",
            _ => "number",
        };
        let v = self.pop(pos, label)?;
        match field {
            FIRE => self.out.fire = Some(truthy(self.m, v)),
            LAUNCH => self.out.launch = Some(truthy(self.m, v)),
            TEXT => self.out.text = Some(to_text(self.m, v)),
            BRAKE => {
                let level = match v {
                    Value::Bool(b) => f64::from(u8::from(b)),
                    _ => max(0.0, min(1.0, self.finite(v, pos, "brake")?)),
                };
                self.out.numbers[BRAKE as usize] = Some(level);
            }
            _ => {
                let n = self.finite(v, pos, if field == VALUE && labelled { "botStatusValue" } else { purpose })?;
                self.out.numbers[field as usize] = Some(n);
            }
        }
        if i.b & 1 != 0 {
            self.owned.push(field);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::number_text;

    #[test]
    fn numbers_read_as_ecmascript_writes_them() {
        for (x, text) in [
            (1e21, "1e+21"),
            (1e20, "100000000000000000000"),
            (1e-7, "1e-7"),
            (1.5e-7, "1.5e-7"),
            (0.000001, "0.000001"),
            (-0.0, "0"),
            (f64::NAN, "NaN"),
            (f64::NEG_INFINITY, "-Infinity"),
            (0.1 + 0.2, "0.30000000000000004"),
            (123.456, "123.456"),
            (-42.0, "-42"),
            (1.7976931348623157e308, "1.7976931348623157e+308"),
            (5e-324, "5e-324"),
        ] {
            assert_eq!(number_text(x), text);
        }
    }
}
