"""The core library and its handle: loading, opening, closing and errors."""
import ctypes
import os

lib = ctypes.CDLL(os.environ.get("CLUNKYARD_LIB", "libclunkyard.so.2"))
_U32 = ctypes.c_uint32
lib.cy_open.argtypes = lib.cy_open_memory.argtypes = [ctypes.c_int32, ctypes.c_int32, ctypes.c_char_p, _U32, ctypes.c_char_p, _U32,
                                                      ctypes.POINTER(ctypes.c_void_p)]
lib.cy_close.argtypes = [ctypes.c_void_p]
lib.cy_last_error.argtypes = [ctypes.c_void_p, ctypes.c_char_p, _U32, ctypes.POINTER(_U32)]


class Error(Exception):
    """The core refused a call. status is its CY_ status, such as E_RANGE."""

    def __init__(self, status, message=""):
        super().__init__(message or f"status {status}")
        self.status = status


def check(status, robot=None):
    if status >= 0:
        return status
    message, length = ctypes.create_string_buffer(512), _U32()
    if robot is not None and lib.cy_last_error(robot, message, 512, ctypes.byref(length)) == 0:
        raise Error(status, message.raw[:length.value].decode(errors="replace"))
    raise Error(status)


def open(name, version, api, memory=False):
    """A robot handle on standard input and output, or on a memory port for tests."""
    robot, name, version = ctypes.c_void_p(), name.encode(), version.encode()
    check((lib.cy_open_memory if memory else lib.cy_open)(api[0], api[1], name, len(name), version, len(version), ctypes.byref(robot)))
    return robot


def close(robot):
    lib.cy_close(robot)
