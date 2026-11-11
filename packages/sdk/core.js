'use strict';
// The robot core's wasm32 build (sdk/core/dist/clunkyard-core.wasm) in
// process, for the DSL interpreters and later the JavaScript binding
// (docs/design/robot-core.md sections 1.4 and 4.2).
//
// Each robot gets its own instance and a memory-port session in it: lines go
// in with feed() and replies come out with reply(). A panic in the core
// traps the whole instance (wasm32 cannot unwind), so every call goes
// through invoke(), which on a WebAssembly.RuntimeError discards the
// instance, instantiates the module afresh, feeds it the kept hello and hands
// it the panic count and the tick that faulted. It then throws a CoreFault.
// The turn is answered as usual with send(), and the core writes an empty
// act for it; the next state plays on a fresh session, as on native. After
// three faults in a match the core answers every state with an empty act.
//
// The module is passed in as bytes or a compiled WebAssembly.Module, so a
// bundle can carry it; loadDefault() reads the committed file in Node.

const { API_VERSION, SIZE: WORDS_SIZE } = require('./words');

const OK = 0, NONE = 1, SHORT = 2;
const EVENTS = [null, 'hello', 'state', 'end'];
const [API_MAJOR, API_MINOR] = API_VERSION.split('.').map(Number);
const encoder = new TextEncoder(), decoder = new TextDecoder();

// A negative status from the core, with its message.
class CoreError extends Error {
  constructor(symbol, status, message) {
    super(`${symbol}: ${message || `status ${status}`}`);
    this.status = status;
  }
}

// The core trapped; the robot now runs on a fresh instance.
class CoreFault extends Error {
  constructor(symbol, cause, panics) {
    super(`a fault in the core (${symbol}); it is not the robot's: ${cause.message}`);
    this.cause = cause;
    this.panics = panics;
  }
}

function compile(source) {
  const module = source instanceof WebAssembly.Module ? source : new WebAssembly.Module(source);
  const imports = WebAssembly.Module.imports(module);
  if (imports.length) throw new Error(`the core imports nothing, but this module imports ${imports.map(item => `${item.module}.${item.name}`).join(', ')}`);
  return module;
}

class CoreRobot {
  constructor(module, { binding = 'clunkyard-dsl', version = API_VERSION } = {}) {
    this.module = module;
    this.binding = binding;
    this.version = version;
    this.panics = 0;
    this.hello = null;
    // The last state line read, for the tick of a fault.
    this.state = null;
    this.fed = null;
    this.instantiate();
  }

  instantiate() {
    this.exports = new WebAssembly.Instance(this.module, {}).exports;
    this.scratchSize = 0;
    this.scratch = 0;
    this.reserve(1 << 16);
    const name = encoder.encode(this.binding), version = encoder.encode(this.version);
    const at = this.write(new Uint8Array([...name, ...version]), 8);
    const status = this.exports.cy_open_memory(API_MAJOR, API_MINOR, at, name.length, at + name.length, version.length, this.scratch);
    if (status !== OK) throw new CoreError('cy_open_memory', status);
    this.handle = this.view().getUint32(this.scratch, true);
  }

  // Memory moves when it grows, so every read takes a fresh view.
  view() { return new DataView(this.exports.memory.buffer); }
  bytes() { return new Uint8Array(this.exports.memory.buffer); }

  // At least size bytes of scratch memory, 8-byte aligned.
  reserve(size) {
    if (size <= this.scratchSize) return;
    if (this.scratch) this.exports.cy_free(this.scratch, this.scratchSize);
    this.scratchSize = Math.max(size, this.scratchSize * 2);
    this.scratch = this.exports.cy_alloc(this.scratchSize);
    if (!this.scratch) throw new Error('the core is out of memory');
  }

  // Copies bytes into scratch memory at offset, giving their address.
  write(bytes, offset = 0) {
    this.reserve(offset + bytes.length);
    this.bytes().set(bytes, this.scratch + offset);
    return this.scratch + offset;
  }

  // Calls an export, recovering from a trap.
  invoke(symbol, ...args) {
    try {
      return this.exports[symbol](...args);
    } catch (error) {
      if (!(error instanceof WebAssembly.RuntimeError)) throw error;
      this.recover(symbol);
      throw new CoreFault(symbol, error, this.panics);
    }
  }

  // A trap in next() came while reading the line just fed, so that line is
  // the hello to keep or the state whose tick to answer.
  recover(symbol) {
    if (symbol === 'cy_session_next' && this.fed !== null) {
      let type = null;
      try { type = JSON.parse(this.fed).type; } catch { /* not JSON: the state stays as it was */ }
      if (type === 'hello') Object.assign(this, { hello: this.fed, state: null, panics: 0 });
      else if (type === 'state') this.state = this.fed;
    }
    this.panics += 1;
    this.instantiate();
    const tick = this.state === null ? -1 : JSON.parse(this.state).tick;
    if (this.hello !== null) {
      try {
        this.feedLine(this.hello);
        this.check('cy_session_next', this.exports.cy_session_next(this.handle, this.scratch));
      } catch (error) {
        // The hello itself traps: carry on without it.
        if (!(error instanceof WebAssembly.RuntimeError)) throw error;
        this.instantiate();
        this.hello = null;
      }
    }
    this.exports.cy_wasm_recover(this.handle, this.panics, tick);
  }

  check(symbol, status) {
    if (status < 0) throw new CoreError(symbol, status, this.lastError());
    return status;
  }

  lastError() {
    const status = this.exports.cy_last_error(this.handle, this.scratch + 8, this.scratchSize - 8, this.scratch);
    return status === OK ? decoder.decode(this.bytes().subarray(this.scratch + 8, this.scratch + 8 + this.view().getUint32(this.scratch, true))) : '';
  }

  feedLine(line) {
    const bytes = encoder.encode(line);
    return this.exports.cy_session_feed(this.handle, this.write(bytes), bytes.length);
  }

  // ---- the session ----

  // Hands the core one line; next() then reads it.
  feed(line) {
    this.fed = line;
    const bytes = encoder.encode(line);
    const at = this.write(bytes);
    this.check('cy_session_feed', this.invoke('cy_session_feed', this.handle, at, bytes.length));
  }

  // 'hello', 'state' or 'end'. The hello is kept for a recovery, and a new
  // one starts the panic count again.
  next() {
    this.check('cy_session_next', this.invoke('cy_session_next', this.handle, this.scratch));
    const event = EVENTS[this.view().getInt32(this.scratch, true)];
    if (event === 'hello') Object.assign(this, { hello: this.fed, state: null, panics: 0 });
    else if (event === 'state') this.state = this.fed;
    return event;
  }

  send() { this.check('cy_session_send', this.invoke('cy_session_send', this.handle)); }

  forfeit(code, message = '') {
    const a = encoder.encode(code), b = encoder.encode(message);
    const at = this.write(new Uint8Array([...a, ...b]));
    this.check('cy_session_forfeit', this.invoke('cy_session_forfeit', this.handle, at, a.length, at + a.length, b.length));
  }

  // The next line the core wrote, or null.
  reply() {
    for (;;) {
      const status = this.check('cy_session_reply', this.invoke('cy_session_reply', this.handle, this.scratch + 8, this.scratchSize - 8, this.scratch));
      if (status === NONE) return null;
      const length = this.view().getUint32(this.scratch, true);
      if (status === SHORT) { this.reserve(length + 8); continue; }
      return decoder.decode(this.bytes().subarray(this.scratch + 8, this.scratch + 8 + length));
    }
  }

  // ---- the DSL ----

  // The words record (cy_dsl_words_v1) for this state, one f64 a word; see
  // src/dsl/words.js read().
  words() {
    this.reserve(WORDS_SIZE);
    this.check('cy_dsl_fill_v1', this.invoke('cy_dsl_fill_v1', this.handle, this.scratch));
    return new Float64Array(this.exports.memory.buffer.slice(this.scratch, this.scratch + WORDS_SIZE));
  }

  // A control's per-field setter (words.js CONTROLS): a number, text, or
  // nothing for a flag. The core clamps and writes the act.
  set(symbol, value) {
    let status;
    if (value === undefined) status = this.invoke(symbol, this.handle);
    else if (typeof value === 'string') {
      const bytes = encoder.encode(value);
      status = this.invoke(symbol, this.handle, this.write(bytes), bytes.length);
    } else status = this.invoke(symbol, this.handle, value);
    return this.check(symbol, status);
  }

  close() {
    this.exports.cy_close(this.handle);
    this.handle = 0;
  }
}

// The core, compiled once; robot() opens one robot on its own instance.
function load(source) {
  const module = compile(source);
  return { module, robot: options => new CoreRobot(module, options) };
}

let committed = null;
function loadDefault() {
  if (!committed) {
    const fs = require('node:fs'), path = require('node:path');
    committed = load(fs.readFileSync(path.join(__dirname, '../../sdk/core/dist/clunkyard-core.wasm')));
  }
  return committed;
}

module.exports = { load, loadDefault, compile, CoreRobot, CoreError, CoreFault, OK, NONE, SHORT };
