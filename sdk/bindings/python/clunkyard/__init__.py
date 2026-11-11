"""Clunkyard robots in Python, on the robot core.

Subclass Robot, override on_start and on_turn, and call run(MyRobot()).
The subsystems are attributes: self.me, self.match, self.sensors, self.gun,
self.drive, self.scanner and the rest, as docs/sdk/names.md lists them.
Controls set only the fields given, so a later call merges with an earlier
one, and drive.get("throttle") reads a field back.
"""
import sys
import traceback

from . import _core
from ._api import *  # noqa: F401,F403 (the records, enums and angles)
from ._api import API, E_INTERNAL, VERSION, attach
from ._core import Error

NAME = "clunkyard-python"


class Forfeit(Exception):
    """Raise it to forfeit the match: code is short and machine-readable."""

    def __init__(self, code, message=""):
        super().__init__(code, message)
        self.code, self.message = code, message


class Robot:
    """A robot. on_start runs once, after the hello; on_turn once a tick."""

    def on_start(self):
        pass

    def on_turn(self):
        pass


def _loop(robot):
    session = robot.session
    while True:
        try:
            event = session.next()
        except Error as error:
            print(f"clunkyard: {error}", file=sys.stderr)
            if error.status == E_INTERNAL:
                try:
                    session.send()
                except Error as send_error:
                    print(f"clunkyard: {send_error}", file=sys.stderr)
                    return
            continue
        if event == "end":
            return
        try:
            robot.on_start() if event == "hello" else robot.on_turn()
        except Forfeit as forfeit:
            session.forfeit(forfeit.code, forfeit.message)
            continue
        except Exception:
            # One bad tick does not lose the match: log it, and answer with
            # what the turn had set before it failed.
            traceback.print_exc()
        session.send()


def run(robot, *, binding_name=NAME):
    """Plays a match on standard input and output. A stray print goes to standard error."""
    attach(robot, _core.open(binding_name, VERSION, API))
    _loop(robot)


def play(robot, lines, *, binding_name=NAME):
    """Plays lines (a hello, then states) in memory, for tests; returns the lines the robot wrote."""
    handle = _core.open(binding_name, VERSION, API, memory=True)
    attach(robot, handle)
    for line in lines:
        robot.session.feed(line)
    _loop(robot)
    out = []
    while (line := robot.session.reply()) is not None:
        out.append(line)
    _core.close(handle)
    return out
