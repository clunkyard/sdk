# Generated from protocol/v2/api.json by protocol/v2/codegen/bindings/python.js; do not edit. Run `make api`.
"""The Clunkyard robot API 2.0.0 over the core's C ABI, generated from protocol/v2/api.json."""
from ctypes import POINTER, Structure, byref, c_char_p, c_double, c_int32, c_uint32, c_void_p, create_string_buffer
from ._core import lib as _lib, check as _check, Error

VERSION = "2.0.0"
API = (2, 0)
OK = 0
NONE = 1
SHORT = 2
E_NOT_FINITE = -1
E_RANGE = -2
E_ARGUMENT = -3
E_PHASE = -4
E_VERSION = -5
E_INTERNAL = -6

_E_SOURCE = {"scan": 1, "proximity": 2, "faint": 3, "beam": 4, "muzzle": 5, "motion": 6, "ping": 7}
_N_SOURCE = {value: wire for wire, value in _E_SOURCE.items()}
_E_MODE = {"2d": 1, "3d": 2}
_N_MODE = {value: wire for wire, value in _E_MODE.items()}
_E_RULESET = {"classic": 1, "inertial-2d": 2, "gravity-flat": 3, "gravity-hills": 4, "zero-g": 5}
_N_RULESET = {value: wire for wire, value in _E_RULESET.items()}
_E_CONTROL = {"direct": 1, "forces": 2}
_N_CONTROL = {value: wire for wire, value in _E_CONTROL.items()}
_E_TERRAIN_KIND = {"none": 1, "flat": 2, "hills": 3}
_N_TERRAIN_KIND = {value: wire for wire, value in _E_TERRAIN_KIND.items()}
_E_SHAPE = {"circle": 1, "sphere": 2, "column": 3}
_N_SHAPE = {value: wire for wire, value in _E_SHAPE.items()}
_E_PICKUP_KIND = {"energy": 1, "health": 2, "armour": 3}
_N_PICKUP_KIND = {value: wire for wire, value in _E_PICKUP_KIND.items()}
_E_COMPUTE_UNIT = {"operations": 1, "fuel": 2, "cpu-ms": 3}
_N_COMPUTE_UNIT = {value: wire for wire, value in _E_COMPUTE_UNIT.items()}
_E_EVENT = {"hello": 1, "state": 2, "end": 3}
_N_EVENT = {value: wire for wire, value in _E_EVENT.items()}

_PD, _PI, _PU = POINTER(c_double), POINTER(c_int32), POINTER(c_uint32)


class _Record(Structure):
    def __repr__(self):
        return f"{type(self).__name__}({', '.join(f'{name}={getattr(self, name)!r}' for name in self._names)})"


def _utf8(text):
    data = str(text).encode()
    return data, len(data)


def _enum_in(names, value):
    return 0 if value is None else names.get(value, -1)


class _c:
    """The core's functions, by their plain names."""


def _fn(name, version, *argtypes):
    fn = getattr(_lib, f"{name}_v{version}" if version else name)
    fn.argtypes, fn.restype = argtypes, c_int32
    setattr(_c, name, fn)


def _robot(args):
    return args[0] if args and isinstance(args[0], c_void_p) else None


def _do(fn, args):
    return _check(fn(*args), _robot(args))


def _num(fn, args, ctype):
    out = ctype()
    return out.value if _do(fn, args + (byref(out),)) == OK else None


def _bool(fn, args):
    value = _num(fn, args, c_int32)
    return None if value is None else bool(value)


def _enum(fn, args, names):
    value = _num(fn, args, c_int32)
    return None if value is None else names.get(value)


def _rec(fn, args, cls):
    out = cls()
    if _do(fn, args + (byref(out),)) != OK:
        return None
    out._h = _robot(args)
    return out


def _list(fn, args, cls, size=16):
    count = c_uint32()
    while True:
        out = (cls * size)()
        if _do(fn, args + (out, size, byref(count))) != SHORT:
            return list(out[:count.value])
        size = count.value


def _text(fn, args, size=256):
    length = c_uint32()
    while True:
        out = create_string_buffer(size)
        status = _do(fn, args + (out, size, byref(length)))
        if status == OK:
            return out.raw[:length.value].decode()
        if status != SHORT:
            return None
        size = length.value + 1


def _get(robot, fields, field):
    if field not in fields:
        raise Error(E_ARGUMENT, f"no field {field!r}; the fields are {', '.join(fields)}")
    fn, text = fields[field]
    return _text(fn, (robot,)) if text else _num(fn, (robot,), c_double)


class Vec3(_Record):
    """A vector in world axes."""
    _fields_ = [("x", c_double), ("y", c_double), ("z", c_double)]
    _names = ("x", "y", "z",)

class Rotation(_Record):
    """A rotation rate or torque about the robot's own axes."""
    _fields_ = [("yaw", c_double), ("pitch", c_double), ("roll", c_double)]
    _names = ("yaw", "pitch", "roll",)

class Direction(_Record):
    """A direction as a heading and an elevation."""
    _fields_ = [("heading", c_double), ("pitch", c_double)]
    _names = ("heading", "pitch",)

class Attitude(_Record):
    """The chassis axes as unit vectors in world axes."""
    _fields_ = [("forward", Vec3), ("right", Vec3), ("up", Vec3)]
    _names = ("forward", "right", "up",)

class Applied(_Record):
    """What the previous tick applied to you, after the battery scaled the drive."""
    _fields_ = [("force", Vec3), ("torque", Rotation), ("brake", c_double)]
    _names = ("force", "torque", "brake",)

class Scan(_Record):
    """The scan that produced this tick's contacts: the one your previous act asked for."""
    _fields_ = [("heading", c_double), ("pitch", c_double), ("arc", c_double), ("range", c_double)]
    _names = ("heading", "pitch", "arc", "range",)

class Contact(_Record):
    """A robot the sensors detect this tick."""
    _fields_ = [("slot", c_int32), ("_source", c_int32), ("range", c_double), ("bearing", c_double), ("elevation", c_double), ("bearing_error", c_double), ("range_error", c_double), ("position", Vec3)]
    _names = ("slot", "source", "range", "bearing", "elevation", "bearing_error", "range_error", "position",)
    source = property(lambda self: _N_SOURCE.get(self._source))

class Cue(_Record):
    """The most accurate passive contact the Tracker holds: where to look, not where to shoot."""
    _fields_ = [("_source", c_int32), ("tick", c_int32), ("bearing_error", c_double), ("position", Vec3)]
    _names = ("source", "tick", "bearing_error", "position",)
    source = property(lambda self: _N_SOURCE.get(self._source))

class Solution(_Record):
    """Where to aim for a shot fired now to meet a target, and when it meets it."""
    _fields_ = [("heading", c_double), ("pitch", c_double), ("time", c_double)]
    _names = ("heading", "pitch", "time",)

class Zone(_Record):
    """The sudden-death safe zone."""
    _fields_ = [("_shape", c_int32), ("reserved", c_int32), ("radius", c_double), ("center", Vec3)]
    _names = ("shape", "radius", "center",)
    shape = property(lambda self: _N_SHAPE.get(self._shape))
    def margin(self, point):
        return _num(_c.cy_nav_zone_margin, (self._h, byref(point)), c_double)

    def contains(self, point, margin=0):
        return _bool(_c.cy_nav_zone_contains, (self._h, byref(point), margin))

    def distance_from(self, point):
        return _num(_c.cy_nav_zone_distance_from, (self._h, byref(point)), c_double)


class Storm(_Record):
    """The storm now."""
    _fields_ = [("_shape", c_int32), ("reserved", c_int32), ("radius", c_double), ("dps", c_double), ("center", Vec3), ("velocity", Vec3)]
    _names = ("shape", "radius", "dps", "center", "velocity",)
    shape = property(lambda self: _N_SHAPE.get(self._shape))
    def margin(self, point):
        return _num(_c.cy_nav_storm_margin, (self._h, byref(point)), c_double)

    def contains(self, point, margin=0):
        return _bool(_c.cy_nav_storm_contains, (self._h, byref(point), margin))

    def distance_from(self, point):
        return _num(_c.cy_nav_storm_distance_from, (self._h, byref(point)), c_double)


class Hill(_Record):
    """The king-of-the-hill zone now."""
    _fields_ = [("_shape", c_int32), ("holder", c_int32), ("_contested", c_int32), ("moves_tick", c_int32), ("held_ticks", c_int32), ("best_held_ticks", c_int32), ("radius", c_double), ("center", Vec3), ("next", Vec3)]
    _names = ("shape", "holder", "contested", "moves_tick", "held_ticks", "best_held_ticks", "radius", "center", "next",)
    shape = property(lambda self: _N_SHAPE.get(self._shape))
    contested = property(lambda self: bool(self._contested))
    def margin(self, point):
        return _num(_c.cy_nav_hill_margin, (self._h, byref(point)), c_double)

    def contains(self, point, margin=0):
        return _bool(_c.cy_nav_hill_contains, (self._h, byref(point), margin))

    def distance_from(self, point):
        return _num(_c.cy_nav_hill_distance_from, (self._h, byref(point)), c_double)


class Pickup(_Record):
    """A cell on the field."""
    _fields_ = [("_kind", c_int32), ("expires_tick", c_int32), ("amount", c_double), ("position", Vec3)]
    _names = ("kind", "expires_tick", "amount", "position",)
    kind = property(lambda self: _N_PICKUP_KIND.get(self._kind))

class Compute(_Record):
    """How the match charges compute against the battery."""
    _fields_ = [("k", c_double), ("quanta", c_int32), ("_unit", c_int32)]
    _names = ("k", "quanta", "unit",)
    unit = property(lambda self: _N_COMPUTE_UNIT.get(self._unit))

_fn("cy_match_mode", 0, c_void_p, _PI)
_fn("cy_match_ruleset", 0, c_void_p, _PI)
_fn("cy_match_control", 0, c_void_p, _PI)
_fn("cy_match_robots", 0, c_void_p, _PI)
_fn("cy_match_is_3d", 0, c_void_p, _PI)
_fn("cy_match_tick_rate", 0, c_void_p, _PI)
_fn("cy_match_max_ticks", 0, c_void_p, _PI)
_fn("cy_robot_id", 0, c_void_p, c_int32, c_char_p, c_uint32, _PU)
_fn("cy_clock_tick", 0, c_void_p, _PI)
_fn("cy_clock_seconds", 0, c_void_p, _PD)
_fn("cy_clock_remaining_ticks", 0, c_void_p, _PI)
_fn("cy_arena_size", 0, c_void_p, POINTER(Vec3))
_fn("cy_arena_gravity", 0, c_void_p, POINTER(Vec3))
_fn("cy_arena_terrain_kind", 0, c_void_p, _PI)
_fn("cy_arena_ground_height", 0, c_void_p, c_double, c_double, _PD)
_fn("cy_me_health", 0, c_void_p, _PD)
_fn("cy_me_position", 0, c_void_p, POINTER(Vec3))
_fn("cy_me_velocity", 0, c_void_p, POINTER(Vec3))
_fn("cy_me_heading", 0, c_void_p, _PD)
_fn("cy_me_pitch", 0, c_void_p, _PD)
_fn("cy_me_roll", 0, c_void_p, _PD)
_fn("cy_me_angular_velocity", 0, c_void_p, POINTER(Rotation))
_fn("cy_me_attitude", 1, c_void_p, POINTER(Attitude))
_fn("cy_me_grounded", 0, c_void_p, _PI)
_fn("cy_me_altitude", 0, c_void_p, _PD)
_fn("cy_me_applied", 1, c_void_p, POINTER(Applied))
_fn("cy_battery_charge", 0, c_void_p, _PD)
_fn("cy_battery_capacity", 0, c_void_p, _PD)
_fn("cy_battery_regen", 0, c_void_p, _PD)
_fn("cy_battery_sustainable_drive", 0, c_void_p, _PD)
_fn("cy_battery_compute", 1, c_void_p, POINTER(Compute))
_fn("cy_battery_drive_cost", 0, c_void_p, c_double, _PD)
_fn("cy_drive_hover_lift", 0, c_void_p, _PD)
_fn("cy_drive_throttle", 0, c_void_p, c_double)
_fn("cy_drive_turn", 0, c_void_p, c_double)
_fn("cy_drive_pitch_turn", 0, c_void_p, c_double)
_fn("cy_drive_thrust", 0, c_void_p, c_double)
_fn("cy_drive_strafe", 0, c_void_p, c_double)
_fn("cy_drive_lift", 0, c_void_p, c_double)
_fn("cy_drive_yaw_torque", 0, c_void_p, c_double)
_fn("cy_drive_pitch_torque", 0, c_void_p, c_double)
_fn("cy_drive_roll_torque", 0, c_void_p, c_double)
_fn("cy_drive_brake", 0, c_void_p, c_double)
_fn("cy_drive_get_throttle", 0, c_void_p, _PD)
_fn("cy_drive_get_turn", 0, c_void_p, _PD)
_fn("cy_drive_get_pitch_turn", 0, c_void_p, _PD)
_fn("cy_drive_get_thrust", 0, c_void_p, _PD)
_fn("cy_drive_get_strafe", 0, c_void_p, _PD)
_fn("cy_drive_get_lift", 0, c_void_p, _PD)
_fn("cy_drive_get_yaw_torque", 0, c_void_p, _PD)
_fn("cy_drive_get_pitch_torque", 0, c_void_p, _PD)
_fn("cy_drive_get_roll_torque", 0, c_void_p, _PD)
_fn("cy_drive_get_brake", 0, c_void_p, _PD)
_fn("cy_gun_aim_now", 0, c_void_p, POINTER(Direction))
_fn("cy_gun_cooldown_ticks", 0, c_void_p, _PI)
_fn("cy_gun_spread", 0, c_void_p, _PD)
_fn("cy_gun_max_power", 0, c_void_p, _PD)
_fn("cy_gun_damage_scale", 0, c_void_p, _PD)
_fn("cy_gun_can_fire", 0, c_void_p, c_double, c_double, _PI)
_fn("cy_gun_shot_speed", 0, c_void_p, c_double, _PD)
_fn("cy_gun_shot_cost", 0, c_void_p, c_double, _PD)
_fn("cy_gun_reload_ticks", 0, c_void_p, c_double, _PD)
_fn("cy_gun_shot_damage", 0, c_void_p, c_double, _PD)
_fn("cy_gun_power_for_speed", 0, c_void_p, c_double, _PD)
_fn("cy_gun_power_for_flight_time", 0, c_void_p, c_double, c_double, _PD)
_fn("cy_gun_lead", 1, c_void_p, POINTER(Contact), c_double, POINTER(Solution))
_fn("cy_gun_lead_point", 1, c_void_p, POINTER(Vec3), POINTER(Vec3), c_double, POINTER(Solution))
_fn("cy_gun_aim_heading", 0, c_void_p, c_double)
_fn("cy_gun_aim_pitch", 0, c_void_p, c_double)
_fn("cy_gun_fire", 0, c_void_p)
_fn("cy_gun_power", 0, c_void_p, c_double)
_fn("cy_gun_get_aim_heading", 0, c_void_p, _PD)
_fn("cy_gun_get_aim_pitch", 0, c_void_p, _PD)
_fn("cy_gun_get_fire", 0, c_void_p, _PD)
_fn("cy_gun_get_power", 0, c_void_p, _PD)
_fn("cy_scanner_last", 1, c_void_p, POINTER(Scan))
_fn("cy_scanner_proximity_range", 0, c_void_p, _PD)
_fn("cy_scanner_reach", 0, c_void_p, c_double, _PD)
_fn("cy_scanner_arc_for_reach", 0, c_void_p, c_double, _PD)
_fn("cy_scanner_heading", 0, c_void_p, c_double)
_fn("cy_scanner_arc", 0, c_void_p, c_double)
_fn("cy_scanner_pitch", 0, c_void_p, c_double)
_fn("cy_scanner_get_heading", 0, c_void_p, _PD)
_fn("cy_scanner_get_arc", 0, c_void_p, _PD)
_fn("cy_scanner_get_pitch", 0, c_void_p, _PD)
_fn("cy_sensors_contacts", 1, c_void_p, POINTER(Contact), c_uint32, _PU)
_fn("cy_sensors_nearest", 1, c_void_p, POINTER(Contact))
_fn("cy_sensors_cues", 1, c_void_p, POINTER(Contact), c_uint32, _PU)
_fn("cy_sensors_contact", 1, c_void_p, c_int32, POINTER(Contact))
_fn("cy_sensors_position_of", 1, c_void_p, POINTER(Contact), POINTER(Vec3))
_fn("cy_tracker_cue", 1, c_void_p, POINTER(Cue))
_fn("cy_tracker_nearest_bearing_rate", 0, c_void_p, _PD)
_fn("cy_tracker_velocity", 0, c_void_p, c_int32, POINTER(Vec3))
_fn("cy_tracker_position", 0, c_void_p, c_int32, POINTER(Vec3))
_fn("cy_tracker_last_seen", 0, c_void_p, c_int32, _PI)
_fn("cy_tracker_configure", 0, c_void_p, c_double, c_int32, c_int32)
_fn("cy_nav_zone", 1, c_void_p, POINTER(Zone))
_fn("cy_nav_zone_margin", 0, c_void_p, POINTER(Vec3), _PD)
_fn("cy_nav_zone_contains", 0, c_void_p, POINTER(Vec3), c_double, _PI)
_fn("cy_nav_zone_distance_from", 0, c_void_p, POINTER(Vec3), _PD)
_fn("cy_nav_storm", 1, c_void_p, POINTER(Storm))
_fn("cy_nav_storm_margin", 0, c_void_p, POINTER(Vec3), _PD)
_fn("cy_nav_storm_contains", 0, c_void_p, POINTER(Vec3), c_double, _PI)
_fn("cy_nav_storm_distance_from", 0, c_void_p, POINTER(Vec3), _PD)
_fn("cy_nav_hill", 1, c_void_p, POINTER(Hill))
_fn("cy_nav_hill_margin", 0, c_void_p, POINTER(Vec3), _PD)
_fn("cy_nav_hill_contains", 0, c_void_p, POINTER(Vec3), c_double, _PI)
_fn("cy_nav_hill_distance_from", 0, c_void_p, POINTER(Vec3), _PD)
_fn("cy_nav_pickups", 1, c_void_p, POINTER(Pickup), c_uint32, _PU)
_fn("cy_nav_nearest_pickup", 1, c_void_p, c_int32, POINTER(Pickup))
_fn("cy_nav_field_distance", 0, c_void_p, POINTER(Vec3), POINTER(Vec3), _PD)
_fn("cy_score_me", 0, c_void_p, _PD)
_fn("cy_score_best", 0, c_void_p, _PD)
_fn("cy_score_armour", 0, c_void_p, _PD)
_fn("cy_angles_normalize", 0, c_double, _PD)
_fn("cy_angles_angle_diff", 0, c_double, c_double, _PD)
_fn("cy_angles_bearing_to", 0, POINTER(Vec3), POINTER(Vec3), _PD)
_fn("cy_angles_elevation_to", 0, POINTER(Vec3), POINTER(Vec3), _PD)
_fn("cy_angles_direction", 0, c_double, c_double, POINTER(Vec3))
_fn("cy_angles_distance", 0, POINTER(Vec3), POINTER(Vec3), _PD)
_fn("cy_angles_clamp", 0, c_double, c_double, c_double, _PD)
_fn("cy_status_text", 0, c_void_p, c_char_p, c_uint32)
_fn("cy_status_value", 0, c_void_p, c_double)
_fn("cy_status_get_text", 0, c_void_p, c_char_p, c_uint32, _PU)
_fn("cy_status_get_value", 0, c_void_p, _PD)
_fn("cy_raw_number", 0, c_void_p, c_char_p, c_uint32, _PD)
_fn("cy_raw_text", 0, c_void_p, c_char_p, c_uint32, c_char_p, c_uint32, _PU)
_fn("cy_raw_has", 0, c_void_p, c_char_p, c_uint32, _PI)
_fn("cy_raw_flag", 0, c_void_p, c_char_p, c_uint32, _PI)
_fn("cy_raw_count", 0, c_void_p, c_char_p, c_uint32, _PI)
_fn("cy_raw_line", 0, c_void_p, c_char_p, c_uint32, _PU)
_fn("cy_raw_extra", 0, c_void_p, c_char_p, c_uint32)
_fn("cy_session_event", 0, c_void_p, _PI)
_fn("cy_session_next", 0, c_void_p, _PI)
_fn("cy_session_send", 0, c_void_p)
_fn("cy_session_forfeit", 0, c_void_p, c_char_p, c_uint32, c_char_p, c_uint32)
_fn("cy_session_log", 0, c_void_p, c_char_p, c_uint32)
_fn("cy_session_feed", 0, c_void_p, c_char_p, c_uint32)
_fn("cy_session_reply", 0, c_void_p, c_char_p, c_uint32, _PU)

class _Match:
    """The match, fixed from the hello."""
    __slots__ = ("_h",)

    def __init__(self, robot):
        self._h = robot

    @property
    def mode(self):
        return _enum(_c.cy_match_mode, (self._h,), _N_MODE)

    @property
    def ruleset(self):
        return _enum(_c.cy_match_ruleset, (self._h,), _N_RULESET)

    @property
    def control(self):
        return _enum(_c.cy_match_control, (self._h,), _N_CONTROL)

    @property
    def robots(self):
        return _num(_c.cy_match_robots, (self._h,), c_int32)

    @property
    def is_3d(self):
        return _bool(_c.cy_match_is_3d, (self._h,))

    @property
    def tick_rate(self):
        return _num(_c.cy_match_tick_rate, (self._h,), c_int32)

    @property
    def max_ticks(self):
        return _num(_c.cy_match_max_ticks, (self._h,), c_int32)

    def robot_id(self, slot):
        return _text(_c.cy_robot_id, (self._h, slot))

class _Clock:
    """Where the match is in time."""
    __slots__ = ("_h",)

    def __init__(self, robot):
        self._h = robot

    @property
    def tick(self):
        return _num(_c.cy_clock_tick, (self._h,), c_int32)

    @property
    def seconds(self):
        return _num(_c.cy_clock_seconds, (self._h,), c_double)

    @property
    def remaining_ticks(self):
        return _num(_c.cy_clock_remaining_ticks, (self._h,), c_int32)

class _Arena:
    """The space robots move in, fixed from the hello."""
    __slots__ = ("_h",)

    def __init__(self, robot):
        self._h = robot

    @property
    def size(self):
        return _rec(_c.cy_arena_size, (self._h,), Vec3)

    @property
    def gravity(self):
        return _rec(_c.cy_arena_gravity, (self._h,), Vec3)

    @property
    def terrain_kind(self):
        return _enum(_c.cy_arena_terrain_kind, (self._h,), _N_TERRAIN_KIND)

    def ground_height(self, x, y):
        return _num(_c.cy_arena_ground_height, (self._h, x, y), c_double)

class _Me:
    """Your robot, after the previous tick resolved."""
    __slots__ = ("_h",)

    def __init__(self, robot):
        self._h = robot

    @property
    def health(self):
        return _num(_c.cy_me_health, (self._h,), c_double)

    @property
    def position(self):
        return _rec(_c.cy_me_position, (self._h,), Vec3)

    @property
    def velocity(self):
        return _rec(_c.cy_me_velocity, (self._h,), Vec3)

    @property
    def heading(self):
        return _num(_c.cy_me_heading, (self._h,), c_double)

    @property
    def pitch(self):
        return _num(_c.cy_me_pitch, (self._h,), c_double)

    @property
    def roll(self):
        return _num(_c.cy_me_roll, (self._h,), c_double)

    @property
    def angular_velocity(self):
        return _rec(_c.cy_me_angular_velocity, (self._h,), Rotation)

    @property
    def attitude(self):
        return _rec(_c.cy_me_attitude, (self._h,), Attitude)

    @property
    def grounded(self):
        return _bool(_c.cy_me_grounded, (self._h,))

    @property
    def altitude(self):
        return _num(_c.cy_me_altitude, (self._h,), c_double)

    @property
    def applied(self):
        return _rec(_c.cy_me_applied, (self._h,), Applied)

class _Battery:
    """The battery that driving and shooting share."""
    __slots__ = ("_h",)

    def __init__(self, robot):
        self._h = robot

    @property
    def charge(self):
        return _num(_c.cy_battery_charge, (self._h,), c_double)

    @property
    def capacity(self):
        return _num(_c.cy_battery_capacity, (self._h,), c_double)

    @property
    def regen(self):
        return _num(_c.cy_battery_regen, (self._h,), c_double)

    @property
    def sustainable_drive(self):
        return _num(_c.cy_battery_sustainable_drive, (self._h,), c_double)

    @property
    def compute(self):
        return _rec(_c.cy_battery_compute, (self._h,), Compute)

    def drive_cost(self, fraction):
        return _num(_c.cy_battery_drive_cost, (self._h, fraction), c_double)

class _Drive:
    """The chassis controls."""
    __slots__ = ("_h",)

    def __init__(self, robot):
        self._h = robot

    @property
    def hover_lift(self):
        return _num(_c.cy_drive_hover_lift, (self._h,), c_double)

    def set(self, throttle=None, turn=None, pitch_turn=None):
        if throttle is not None:
            _do(_c.cy_drive_throttle, (self._h, throttle))
        if turn is not None:
            _do(_c.cy_drive_turn, (self._h, turn))
        if pitch_turn is not None:
            _do(_c.cy_drive_pitch_turn, (self._h, pitch_turn))

    def forces(self, thrust=None, strafe=None, lift=None, yaw_torque=None, pitch_torque=None, roll_torque=None, brake=None):
        if thrust is not None:
            _do(_c.cy_drive_thrust, (self._h, thrust))
        if strafe is not None:
            _do(_c.cy_drive_strafe, (self._h, strafe))
        if lift is not None:
            _do(_c.cy_drive_lift, (self._h, lift))
        if yaw_torque is not None:
            _do(_c.cy_drive_yaw_torque, (self._h, yaw_torque))
        if pitch_torque is not None:
            _do(_c.cy_drive_pitch_torque, (self._h, pitch_torque))
        if roll_torque is not None:
            _do(_c.cy_drive_roll_torque, (self._h, roll_torque))
        if brake is not None:
            _do(_c.cy_drive_brake, (self._h, brake))

    _fields = {"throttle": (_c.cy_drive_get_throttle, False), "turn": (_c.cy_drive_get_turn, False), "pitch_turn": (_c.cy_drive_get_pitch_turn, False), "thrust": (_c.cy_drive_get_thrust, False), "strafe": (_c.cy_drive_get_strafe, False), "lift": (_c.cy_drive_get_lift, False), "yaw_torque": (_c.cy_drive_get_yaw_torque, False), "pitch_torque": (_c.cy_drive_get_pitch_torque, False), "roll_torque": (_c.cy_drive_get_roll_torque, False), "brake": (_c.cy_drive_get_brake, False)}

    def get(self, field):
        """The field's value as set this tick, or None."""
        return _get(self._h, self._fields, field)

class _Gun:
    """The turret and its shots."""
    __slots__ = ("_h",)

    def __init__(self, robot):
        self._h = robot

    @property
    def aim_now(self):
        return _rec(_c.cy_gun_aim_now, (self._h,), Direction)

    @property
    def cooldown_ticks(self):
        return _num(_c.cy_gun_cooldown_ticks, (self._h,), c_int32)

    @property
    def spread(self):
        return _num(_c.cy_gun_spread, (self._h,), c_double)

    @property
    def max_power(self):
        return _num(_c.cy_gun_max_power, (self._h,), c_double)

    @property
    def damage_scale(self):
        return _num(_c.cy_gun_damage_scale, (self._h,), c_double)

    def can_fire(self, power, reserve):
        return _bool(_c.cy_gun_can_fire, (self._h, power, reserve))

    def shot_speed(self, power):
        return _num(_c.cy_gun_shot_speed, (self._h, power), c_double)

    def shot_cost(self, power):
        return _num(_c.cy_gun_shot_cost, (self._h, power), c_double)

    def reload_ticks(self, power):
        return _num(_c.cy_gun_reload_ticks, (self._h, power), c_double)

    def shot_damage(self, power):
        return _num(_c.cy_gun_shot_damage, (self._h, power), c_double)

    def power_for_speed(self, speed):
        return _num(_c.cy_gun_power_for_speed, (self._h, speed), c_double)

    def power_for_flight_time(self, range, seconds):
        return _num(_c.cy_gun_power_for_flight_time, (self._h, range, seconds), c_double)

    def lead(self, contact, power):
        return _rec(_c.cy_gun_lead, (self._h, byref(contact), power), Solution)

    def lead_point(self, position, velocity, power):
        return _rec(_c.cy_gun_lead_point, (self._h, byref(position), byref(velocity), power), Solution)

    def aim(self, heading=None, pitch=None):
        if heading is not None:
            _do(_c.cy_gun_aim_heading, (self._h, heading))
        if pitch is not None:
            _do(_c.cy_gun_aim_pitch, (self._h, pitch))

    def fire(self, power=None):
        if power is not None:
            _do(_c.cy_gun_power, (self._h, power))
        _do(_c.cy_gun_fire, (self._h,))

    _fields = {"heading": (_c.cy_gun_get_aim_heading, False), "pitch": (_c.cy_gun_get_aim_pitch, False), "fire": (_c.cy_gun_get_fire, False), "power": (_c.cy_gun_get_power, False)}

    def get(self, field):
        """The field's value as set this tick, or None."""
        return _get(self._h, self._fields, field)

class _Scanner:
    """The scanner."""
    __slots__ = ("_h",)

    def __init__(self, robot):
        self._h = robot

    @property
    def last(self):
        return _rec(_c.cy_scanner_last, (self._h,), Scan)

    @property
    def proximity_range(self):
        return _num(_c.cy_scanner_proximity_range, (self._h,), c_double)

    def reach(self, arc):
        return _num(_c.cy_scanner_reach, (self._h, arc), c_double)

    def arc_for_reach(self, range):
        return _num(_c.cy_scanner_arc_for_reach, (self._h, range), c_double)

    def set(self, heading=None, arc=None, pitch=None):
        if heading is not None:
            _do(_c.cy_scanner_heading, (self._h, heading))
        if arc is not None:
            _do(_c.cy_scanner_arc, (self._h, arc))
        if pitch is not None:
            _do(_c.cy_scanner_pitch, (self._h, pitch))

    _fields = {"heading": (_c.cy_scanner_get_heading, False), "arc": (_c.cy_scanner_get_arc, False), "pitch": (_c.cy_scanner_get_pitch, False)}

    def get(self, field):
        """The field's value as set this tick, or None."""
        return _get(self._h, self._fields, field)

class _Sensors:
    """What the sensors detect this tick."""
    __slots__ = ("_h",)

    def __init__(self, robot):
        self._h = robot

    @property
    def contacts(self):
        return _list(_c.cy_sensors_contacts, (self._h,), Contact)

    @property
    def nearest(self):
        return _rec(_c.cy_sensors_nearest, (self._h,), Contact)

    @property
    def cues(self):
        return _list(_c.cy_sensors_cues, (self._h,), Contact)

    def contact(self, slot):
        return _rec(_c.cy_sensors_contact, (self._h, slot), Contact)

    def position_of(self, contact):
        return _rec(_c.cy_sensors_position_of, (self._h, byref(contact)), Vec3)

class _Tracker:
    """Estimates from past sightings, updated once a state inside the core."""
    __slots__ = ("_h",)

    def __init__(self, robot):
        self._h = robot

    @property
    def cue(self):
        return _rec(_c.cy_tracker_cue, (self._h,), Cue)

    @property
    def nearest_bearing_rate(self):
        return _num(_c.cy_tracker_nearest_bearing_rate, (self._h,), c_double)

    def velocity(self, slot):
        return _rec(_c.cy_tracker_velocity, (self._h, slot), Vec3)

    def position(self, slot):
        return _rec(_c.cy_tracker_position, (self._h, slot), Vec3)

    def last_seen(self, slot):
        return _num(_c.cy_tracker_last_seen, (self._h, slot), c_int32)

    def configure(self, smoothing=0.5, forget_ticks=40, cue_ticks=60):
        return _do(_c.cy_tracker_configure, (self._h, smoothing, forget_ticks, cue_ticks))

class _Nav:
    """The regions and objects on the field."""
    __slots__ = ("_h",)

    def __init__(self, robot):
        self._h = robot

    @property
    def zone(self):
        return _rec(_c.cy_nav_zone, (self._h,), Zone)

    @property
    def storm(self):
        return _rec(_c.cy_nav_storm, (self._h,), Storm)

    @property
    def hill(self):
        return _rec(_c.cy_nav_hill, (self._h,), Hill)

    @property
    def pickups(self):
        return _list(_c.cy_nav_pickups, (self._h,), Pickup)

    def nearest_pickup(self, kind=None):
        return _rec(_c.cy_nav_nearest_pickup, (self._h, _enum_in(_E_PICKUP_KIND, kind)), Pickup)

    def field_distance(self, a, b):
        return _num(_c.cy_nav_field_distance, (self._h, byref(a), byref(b)), c_double)

class _Score:
    """Time-limit scoring, when the match scores."""
    __slots__ = ("_h",)

    def __init__(self, robot):
        self._h = robot

    @property
    def me(self):
        return _num(_c.cy_score_me, (self._h,), c_double)

    @property
    def best(self):
        return _num(_c.cy_score_best, (self._h,), c_double)

    @property
    def armour(self):
        return _num(_c.cy_score_armour, (self._h,), c_double)

class _Angles:
    """Angle and vector arithmetic in the core's maths, so every language agrees bit for bit."""
    __slots__ = ("_h",)

    def __init__(self, robot):
        self._h = robot

    def normalize(self, degrees):
        return _num(_c.cy_angles_normalize, (degrees,), c_double)

    def angle_diff(self, target, current):
        return _num(_c.cy_angles_angle_diff, (target, current), c_double)

    def bearing_to(self, from_, to):
        return _num(_c.cy_angles_bearing_to, (byref(from_), byref(to)), c_double)

    def elevation_to(self, from_, to):
        return _num(_c.cy_angles_elevation_to, (byref(from_), byref(to)), c_double)

    def direction(self, heading, pitch):
        return _rec(_c.cy_angles_direction, (heading, pitch), Vec3)

    def distance(self, a, b):
        return _num(_c.cy_angles_distance, (byref(a), byref(b)), c_double)

    def clamp(self, value, low=-1, high=1):
        return _num(_c.cy_angles_clamp, (value, low, high), c_double)

class _Status:
    """The status line spectators see."""
    __slots__ = ("_h",)

    def __init__(self, robot):
        self._h = robot

    def set(self, text=None, value=None):
        if text is not None:
            _do(_c.cy_status_text, (self._h, *_utf8(text)))
        if value is not None:
            _do(_c.cy_status_value, (self._h, value))

    _fields = {"text": (_c.cy_status_get_text, True), "value": (_c.cy_status_get_value, False)}

    def get(self, field):
        """The field's value as set this tick, or None."""
        return _get(self._h, self._fields, field)

class _Raw:
    """The messages as the arena sent them, for what the API does not cover."""
    __slots__ = ("_h",)

    def __init__(self, robot):
        self._h = robot

    def number(self, path):
        return _num(_c.cy_raw_number, (self._h, *_utf8(path)), c_double)

    def text(self, path):
        return _text(_c.cy_raw_text, (self._h, *_utf8(path)))

    def has(self, path):
        return _bool(_c.cy_raw_has, (self._h, *_utf8(path)))

    def flag(self, path):
        return _bool(_c.cy_raw_flag, (self._h, *_utf8(path)))

    def count(self, path):
        return _num(_c.cy_raw_count, (self._h, *_utf8(path)), c_int32)

    def line(self):
        return _text(_c.cy_raw_line, (self._h,))

    def extra(self, json):
        return _do(_c.cy_raw_extra, (self._h, *_utf8(json)))

class _Session:
    """The protocol session: hello, ready, state, act and forfeit."""
    __slots__ = ("_h",)

    def __init__(self, robot):
        self._h = robot

    @property
    def event(self):
        return _enum(_c.cy_session_event, (self._h,), _N_EVENT)

    def next(self):
        return _enum(_c.cy_session_next, (self._h,), _N_EVENT)

    def send(self):
        return _do(_c.cy_session_send, (self._h,))

    def forfeit(self, code, message):
        return _do(_c.cy_session_forfeit, (self._h, *_utf8(code), *_utf8(message)))

    def log(self, line):
        return _do(_c.cy_session_log, (self._h, *_utf8(line)))

    def feed(self, line):
        return _do(_c.cy_session_feed, (self._h, *_utf8(line)))

    def reply(self):
        return _text(_c.cy_session_reply, (self._h,))

angles = _Angles(None)

def attach(robot, handle):
    """Gives robot its subsystems, on the core robot handle."""
    robot.match = _Match(handle)
    robot.clock = _Clock(handle)
    robot.arena = _Arena(handle)
    robot.me = _Me(handle)
    robot.battery = _Battery(handle)
    robot.drive = _Drive(handle)
    robot.gun = _Gun(handle)
    robot.scanner = _Scanner(handle)
    robot.sensors = _Sensors(handle)
    robot.tracker = _Tracker(handle)
    robot.nav = _Nav(handle)
    robot.score = _Score(handle)
    robot.angles = angles
    robot.status = _Status(handle)
    robot.raw = _Raw(handle)
    robot.session = _Session(handle)
