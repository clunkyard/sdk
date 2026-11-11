// Generated from protocol/v2/api.json; do not edit. Run make api.

export interface Vec3 {
  x: number;
  y: number;
  z: number;
}
export interface Rotation {
  yaw: number;
  pitch: number;
  roll: number;
}
export interface Direction {
  heading: number;
  pitch: number;
}
export interface Attitude {
  forward: Vec3;
  right: Vec3;
  up: Vec3;
}
export interface Applied {
  force: Vec3;
  torque: Rotation;
  brake: number;
}
export interface Scan {
  heading: number;
  pitch: number;
  arc: number;
  range: number;
}
export interface Contact {
  slot: number;
  source: "scan" | "proximity" | "faint" | "beam" | "muzzle" | "motion" | "ping";
  range: number;
  bearing: number;
  elevation: number;
  bearingError: number;
  rangeError: number;
  position: Vec3;
}
export interface Cue {
  source: "scan" | "proximity" | "faint" | "beam" | "muzzle" | "motion" | "ping";
  tick: number;
  bearingError: number;
  position: Vec3;
}
export interface Solution {
  heading: number;
  pitch: number;
  time: number;
}
export interface Zone {
  shape: "circle" | "sphere" | "column";
  radius: number;
  center: Vec3;
  margin(point: Vec3): number | null;
  contains(point: Vec3, margin?: number): boolean | null;
  distanceFrom(point: Vec3): number | null;
}
export interface Storm {
  shape: "circle" | "sphere" | "column";
  radius: number;
  dps: number;
  center: Vec3;
  velocity: Vec3;
  margin(point: Vec3): number | null;
  contains(point: Vec3, margin?: number): boolean | null;
  distanceFrom(point: Vec3): number | null;
}
export interface Hill {
  shape: "circle" | "sphere" | "column";
  holder: number;
  contested: boolean;
  movesTick: number;
  heldTicks: number;
  bestHeldTicks: number;
  radius: number;
  center: Vec3;
  next: Vec3;
  margin(point: Vec3): number | null;
  contains(point: Vec3, margin?: number): boolean | null;
  distanceFrom(point: Vec3): number | null;
}
export interface Pickup {
  kind: "energy" | "health" | "armour";
  expiresTick: number;
  amount: number;
  position: Vec3;
}
export interface Compute {
  k: number;
  quanta: number;
  unit: "operations" | "fuel" | "cpu-ms";
}
export interface Match {
  readonly mode: "2d" | "3d";
  readonly ruleset: "classic" | "inertial-2d" | "gravity-flat" | "gravity-hills" | "zero-g";
  readonly control: "direct" | "forces";
  readonly robots: number;
  readonly is3d: boolean;
  readonly tickRate: number;
  readonly maxTicks: number;
  robotId(slot: number): string;
}
export interface Clock {
  readonly tick: number;
  readonly seconds: number;
  readonly remainingTicks: number;
}
export interface Arena {
  readonly size: Vec3;
  readonly gravity: Vec3;
  readonly terrainKind: "none" | "flat" | "hills";
  groundHeight(x: number, y: number): number;
}
export interface Me {
  readonly health: number;
  readonly position: Vec3;
  readonly velocity: Vec3;
  readonly heading: number;
  readonly pitch: number;
  readonly roll: number;
  readonly angularVelocity: Rotation;
  readonly attitude: Attitude;
  readonly grounded: boolean;
  readonly altitude: number;
  readonly applied: Applied;
}
export interface Battery {
  readonly charge: number;
  readonly capacity: number;
  readonly regen: number;
  readonly sustainableDrive: number;
  readonly compute: Compute | null;
  driveCost(fraction: number): number;
}
export interface Drive {
  readonly hoverLift: number;
  set(fields?: { throttle?: number; turn?: number; pitchTurn?: number }): void;
  forces(fields?: { thrust?: number; strafe?: number; lift?: number; yawTorque?: number; pitchTorque?: number; rollTorque?: number; brake?: number }): void;
  get(field: "throttle" | "turn" | "pitchTurn" | "thrust" | "strafe" | "lift" | "yawTorque" | "pitchTorque" | "rollTorque" | "brake"): number | string | null;
}
export interface Gun {
  readonly aimNow: Direction;
  readonly cooldownTicks: number;
  readonly spread: number;
  readonly maxPower: number;
  readonly damageScale: number;
  canFire(power: number, reserve: number): boolean;
  shotSpeed(power: number): number;
  shotCost(power: number): number;
  reloadTicks(power: number): number;
  shotDamage(power: number): number;
  powerForSpeed(speed: number): number;
  powerForFlightTime(range: number, seconds: number): number;
  lead(contact: Contact, power: number): Solution | null;
  leadPoint(position: Vec3, velocity: Vec3, power: number): Solution | null;
  aim(fields?: { heading?: number; pitch?: number }): void;
  fire(fields?: { power?: number }): void;
  get(field: "heading" | "pitch" | "fire" | "power"): number | string | null;
}
export interface Scanner {
  readonly last: Scan;
  readonly proximityRange: number;
  reach(arc: number): number;
  arcForReach(range: number): number;
  set(fields?: { heading?: number; arc?: number; pitch?: number }): void;
  get(field: "heading" | "arc" | "pitch"): number | string | null;
}
export interface Sensors {
  readonly contacts: Array<Contact>;
  readonly nearest: Contact | null;
  readonly cues: Array<Contact>;
  contact(slot: number): Contact | null;
  positionOf(contact: Contact): Vec3;
}
export interface Tracker {
  readonly cue: Cue | null;
  readonly nearestBearingRate: number;
  velocity(slot: number): Vec3;
  position(slot: number): Vec3 | null;
  lastSeen(slot: number): number | null;
  configure(smoothing?: number, forgetTicks?: number, cueTicks?: number): void;
}
export interface Nav {
  readonly zone: Zone | null;
  readonly storm: Storm | null;
  readonly hill: Hill | null;
  readonly pickups: Array<Pickup>;
  nearestPickup(kind?: "energy" | "health" | "armour"): Pickup | null;
  fieldDistance(a: Vec3, b: Vec3): number;
}
export interface Score {
  readonly me: number | null;
  readonly best: number | null;
  readonly armour: number | null;
}
export interface Angles {
  normalize(degrees: number): number;
  angleDiff(target: number, current: number): number;
  bearingTo(from: Vec3, to: Vec3): number;
  elevationTo(from: Vec3, to: Vec3): number;
  direction(heading: number, pitch: number): Vec3;
  distance(a: Vec3, b: Vec3): number;
  clamp(value: number, low?: number, high?: number): number;
}
export interface Status {
  set(fields?: { text?: string; value?: number }): void;
  get(field: "text" | "value"): number | string | null;
}
export interface Raw {
  number(path: string): number | null;
  text(path: string): string | null;
  has(path: string): boolean;
  flag(path: string): boolean | null;
  count(path: string): number | null;
  line(): string;
  extra(json: string): void;
}
export interface Session {
  readonly event: "hello" | "state" | "end";
  next(): "hello" | "state" | "end";
  send(): void;
  forfeit(code: string, message: string): void;
  log(line: string): void;
  feed(line: string): void;
  reply(): string | null;
}
export class Robot {
  match: Match;
  clock: Clock;
  arena: Arena;
  me: Me;
  battery: Battery;
  drive: Drive;
  gun: Gun;
  scanner: Scanner;
  sensors: Sensors;
  tracker: Tracker;
  nav: Nav;
  score: Score;
  angles: Angles;
  status: Status;
  raw: Raw;
  session: Session;
  onStart(): void;
  onTurn(): void;
}
export class Forfeit extends Error { constructor(code: string, message?: string); }
export const angles: Angles;
export function run(robot: Robot, options?: { bindingName?: string }): Promise<void>;
export function play(robot: Robot, lines: Iterable<string>, options?: { bindingName?: string }): string[];
