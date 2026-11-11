"""The core can fault while reading a state; the robot must still play on."""
from types import SimpleNamespace
import clunkyard as cy
from clunkyard._api import E_INTERNAL
from clunkyard._core import Error


class Session:
    def __init__(self):
        self.events = iter(["hello", "state", Error(E_INTERNAL, "fault"), "state", "end"])
        self.sent = 0

    def next(self):
        event = next(self.events)
        if isinstance(event, Error):
            raise event
        return event

    def send(self):
        self.sent += 1


session, turns = Session(), []
cy._loop(SimpleNamespace(session=session, on_start=lambda: None, on_turn=lambda: turns.append("turn")))
assert turns == ["turn", "turn"], "the next state must run after a core fault"
assert session.sent == 4, "the faulted tick must receive the core's empty act"
