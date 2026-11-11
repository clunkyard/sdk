# Generated from protocol/v2/api.json by protocol/v2/codegen/bindings/python.js; do not edit. Run `make api`.
"""The binding against every API vector, exact against the core:
node sdk/bindings/conformance.js python3 sdk/bindings/python/test_conformance.py"""
import keyword
import struct
import sys

import clunkyard as cy
from clunkyard import _api, _core

CALLS = {
    "match.mode": ([], "enum:mode"),
    "match.ruleset": ([], "enum:ruleset"),
    "match.control": ([], "enum:control"),
    "match.robots": ([], "i32"),
    "match.is_3d": ([], "bool"),
    "match.tick_rate": ([], "i32"),
    "match.max_ticks": ([], "i32"),
    "match.robot_id": (["i32"], "text"),
    "clock.tick": ([], "i32"),
    "clock.seconds": ([], "f64"),
    "clock.remaining_ticks": ([], "i32"),
    "arena.size": ([], "record:vec3"),
    "arena.gravity": ([], "record:vec3"),
    "arena.terrain_kind": ([], "enum:terrain_kind"),
    "arena.ground_height": (["f64", "f64"], "f64"),
    "me.health": ([], "f64"),
    "me.position": ([], "record:vec3"),
    "me.velocity": ([], "record:vec3"),
    "me.heading": ([], "f64"),
    "me.pitch": ([], "f64"),
    "me.roll": ([], "f64"),
    "me.angular_velocity": ([], "record:rotation"),
    "me.attitude": ([], "record:attitude"),
    "me.grounded": ([], "bool"),
    "me.altitude": ([], "f64"),
    "me.applied": ([], "record:applied"),
    "battery.charge": ([], "f64"),
    "battery.capacity": ([], "f64"),
    "battery.regen": ([], "f64"),
    "battery.sustainable_drive": ([], "f64"),
    "battery.compute": ([], "record:compute"),
    "battery.drive_cost": (["f64"], "f64"),
    "drive.hover_lift": ([], "f64"),
    "gun.aim_now": ([], "record:direction"),
    "gun.cooldown_ticks": ([], "i32"),
    "gun.spread": ([], "f64"),
    "gun.max_power": ([], "f64"),
    "gun.damage_scale": ([], "f64"),
    "gun.can_fire": (["f64", "f64"], "bool"),
    "gun.shot_speed": (["f64"], "f64"),
    "gun.shot_cost": (["f64"], "f64"),
    "gun.reload_ticks": (["f64"], "f64"),
    "gun.shot_damage": (["f64"], "f64"),
    "gun.power_for_speed": (["f64"], "f64"),
    "gun.power_for_flight_time": (["f64", "f64"], "f64"),
    "gun.lead": (["record:contact", "f64"], "record:solution"),
    "gun.lead_point": (["record:vec3", "record:vec3", "f64"], "record:solution"),
    "scanner.last": ([], "record:scan"),
    "scanner.proximity_range": ([], "f64"),
    "scanner.reach": (["f64"], "f64"),
    "scanner.arc_for_reach": (["f64"], "f64"),
    "sensors.contacts": ([], "list:contact"),
    "sensors.nearest": ([], "record:contact"),
    "sensors.cues": ([], "list:contact"),
    "sensors.contact": (["i32"], "record:contact"),
    "sensors.position_of": (["record:contact"], "record:vec3"),
    "tracker.cue": ([], "record:cue"),
    "tracker.nearest_bearing_rate": ([], "f64"),
    "tracker.velocity": (["i32"], "record:vec3"),
    "tracker.position": (["i32"], "record:vec3"),
    "tracker.last_seen": (["i32"], "i32"),
    "tracker.configure": (["f64", "i32", "i32"], None),
    "nav.zone": ([], "record:zone"),
    "nav.zone.margin": (["record:vec3"], "f64"),
    "nav.zone.contains": (["record:vec3", "f64"], "bool"),
    "nav.zone.distance_from": (["record:vec3"], "f64"),
    "nav.storm": ([], "record:storm"),
    "nav.storm.margin": (["record:vec3"], "f64"),
    "nav.storm.contains": (["record:vec3", "f64"], "bool"),
    "nav.storm.distance_from": (["record:vec3"], "f64"),
    "nav.hill": ([], "record:hill"),
    "nav.hill.margin": (["record:vec3"], "f64"),
    "nav.hill.contains": (["record:vec3", "f64"], "bool"),
    "nav.hill.distance_from": (["record:vec3"], "f64"),
    "nav.pickups": ([], "list:pickup"),
    "nav.nearest_pickup": (["enum:pickup_kind"], "record:pickup"),
    "nav.field_distance": (["record:vec3", "record:vec3"], "f64"),
    "score.me": ([], "f64"),
    "score.best": ([], "f64"),
    "score.armour": ([], "f64"),
    "angles.normalize": (["f64"], "f64"),
    "angles.angle_diff": (["f64", "f64"], "f64"),
    "angles.bearing_to": (["record:vec3", "record:vec3"], "f64"),
    "angles.elevation_to": (["record:vec3", "record:vec3"], "f64"),
    "angles.direction": (["f64", "f64"], "record:vec3"),
    "angles.distance": (["record:vec3", "record:vec3"], "f64"),
    "angles.clamp": (["f64", "f64", "f64"], "f64"),
    "raw.number": (["text"], "f64"),
    "raw.text": (["text"], "text"),
    "raw.has": (["text"], "bool"),
    "raw.flag": (["text"], "bool"),
    "raw.count": (["text"], "i32"),
    "raw.line": ([], "text"),
    "raw.extra": (["text"], None),
}
RECORDS = {
    "vec3": (_api.Vec3, [("x", "f64"), ("y", "f64"), ("z", "f64")]),
    "rotation": (_api.Rotation, [("yaw", "f64"), ("pitch", "f64"), ("roll", "f64")]),
    "direction": (_api.Direction, [("heading", "f64"), ("pitch", "f64")]),
    "attitude": (_api.Attitude, [("forward", "record:vec3"), ("right", "record:vec3"), ("up", "record:vec3")]),
    "applied": (_api.Applied, [("force", "record:vec3"), ("torque", "record:rotation"), ("brake", "f64")]),
    "scan": (_api.Scan, [("heading", "f64"), ("pitch", "f64"), ("arc", "f64"), ("range", "f64")]),
    "contact": (_api.Contact, [("slot", "i32"), ("source", "enum:source"), ("range", "f64"), ("bearing", "f64"), ("elevation", "f64"), ("bearing_error", "f64"), ("range_error", "f64"), ("position", "record:vec3")]),
    "cue": (_api.Cue, [("source", "enum:source"), ("tick", "i32"), ("bearing_error", "f64"), ("position", "record:vec3")]),
    "solution": (_api.Solution, [("heading", "f64"), ("pitch", "f64"), ("time", "f64")]),
    "zone": (_api.Zone, [("shape", "enum:shape"), ("reserved", "i32"), ("radius", "f64"), ("center", "record:vec3")]),
    "storm": (_api.Storm, [("shape", "enum:shape"), ("reserved", "i32"), ("radius", "f64"), ("dps", "f64"), ("center", "record:vec3"), ("velocity", "record:vec3")]),
    "hill": (_api.Hill, [("shape", "enum:shape"), ("holder", "i32"), ("contested", "bool"), ("moves_tick", "i32"), ("held_ticks", "i32"), ("best_held_ticks", "i32"), ("radius", "f64"), ("center", "record:vec3"), ("next", "record:vec3")]),
    "pickup": (_api.Pickup, [("kind", "enum:pickup_kind"), ("expires_tick", "i32"), ("amount", "f64"), ("position", "record:vec3")]),
    "compute": (_api.Compute, [("k", "f64"), ("quanta", "i32"), ("unit", "enum:compute_unit")]),
}


def names(kind):
    return getattr(_api, "_N_" + kind[5:].upper())


def value(kind, tokens):
    """An argument from its tokens; an enum's unknown value is a name the binding does not know."""
    if kind.startswith("record:"):
        cls, fields = RECORDS[kind[7:]]
        out = cls()
        for name, field in fields:
            raw = name if field in ("f64", "i32") or field.startswith("record:") else "_" + name
            setattr(out, raw, value(field, tokens) if field.startswith("record:") else value("f64" if field == "f64" else "i32", tokens))
        return out
    token = tokens.pop(0)
    if kind.startswith("enum:"):
        return None if token == "i0" else names(kind).get(int(token[1:]), "?")
    return struct.unpack(">d", bytes.fromhex(token[1:]))[0] if kind == "f64" else int(token[1:]) if kind == "i32" else bytes.fromhex(token[1:]).decode()


def canon(kind, v):
    if v is None:
        return "n"
    if kind.startswith("list:"):
        return "[" + ",".join(canon("record:" + kind[5:], item) for item in v) + "]"
    if kind.startswith("record:"):
        return "{" + ",".join(f"{name}={canon(field, getattr(v, name))}" for name, field in RECORDS[kind[7:]][1] if name != "reserved") + "}"
    if isinstance(v, bool):
        return "t" if v else "f"
    if isinstance(v, str):
        return "s" + v.encode().hex()
    return "7ff8000000000000" if v != v else struct.pack(">d", float(v)).hex()


def answer(label, run, kind):
    try:
        v = run()
    except cy.Error as error:
        return print(label, error.status)
    print(label, 0 if kind is None else 1 if v is None else f"0 {canon(kind, v)}")


def find(bot, name):
    """The object that holds an entry, and the entry's member name; None when a region is absent."""
    *path, last = name.split(".")
    at = bot
    for part in path:
        at = getattr(at, part)
    return at, last


def run(bot, name, tokens):
    kinds, kind = CALLS[name]
    args = [value(k, tokens) for k in kinds]
    at, last = find(bot, name)
    if at is None:
        return lambda: None
    if isinstance(getattr(type(at), last, None), property):
        return lambda: getattr(at, last)
    return lambda: getattr(at, last)(*args)


def main():
    bot = None
    for line in sys.stdin:
        op, *rest = line.rstrip("\n").split("\t")
        if op == "open":
            bot = cy.Robot()
            _api.attach(bot, _core.open(cy.NAME, _api.VERSION, _api.API, memory=True))
        elif op == "close":
            _core.close(bot.session._h)
        elif op == "feed":
            bot.session.feed(bytes.fromhex(rest[0][1:]).decode())
        elif op in ("next", "send", "reply"):
            getattr(bot.session, op)()
        elif op == "call" and rest[0] == "-":
            run(bot, rest[1], rest[2:])()
        elif op == "call":
            answer(rest[0], run(bot, rest[1], rest[2:]), CALLS[rest[1]][1])
        elif op == "set":
            label, name, *field = rest
            kwargs = {field[0] + "_" * keyword.iskeyword(field[0]): value("text" if field[1][0] == "s" else "f64", [field[1]])} if field else {}
            answer(label, lambda: getattr(*find(bot, name))(**kwargs), None)
        elif op == "get":
            answer(rest[0], lambda: getattr(*find(bot, rest[1]))(rest[2]), "value")
        elif op == "act":
            bot.session.send()
            print(rest[0], canon("text", bot.session.reply()))
        elif op == "slots":
            print(rest[0], "[" + ",".join(canon("text", bot.match.robot_id(slot)) for slot in range(1, bot.match.robots + 1)) + "]")


main()
