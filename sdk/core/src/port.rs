//! The port: how the session reads and writes its frames (design section
//! 1.6). The session never touches a file descriptor.

use std::collections::VecDeque;

pub enum Frame {
    Data,
    End,
}

#[derive(Debug)]
pub struct PortError(pub String);

pub trait Port {
    /// Reads the next line, without its newline, into buf.
    fn read_frame(&mut self, buf: &mut Vec<u8>) -> Result<Frame, PortError>;
    /// Writes one line; the port adds the newline.
    fn write_frame(&mut self, bytes: &[u8]) -> Result<(), PortError>;
    /// The memory port, for session.feed and session.reply.
    fn memory(&mut self) -> Option<&mut MemoryPort> {
        None
    }
    fn memory_ref(&self) -> Option<&MemoryPort> {
        None
    }
}

/// Lines handed in by session.feed and taken by session.reply: for tests,
/// play() and event-loop runtimes that do their own I/O.
#[derive(Default)]
pub struct MemoryPort {
    pub input: VecDeque<Vec<u8>>,
    pub output: VecDeque<String>,
}

impl Port for MemoryPort {
    fn read_frame(&mut self, buf: &mut Vec<u8>) -> Result<Frame, PortError> {
        match self.input.pop_front() {
            Some(line) => {
                *buf = line;
                Ok(Frame::Data)
            }
            None => Ok(Frame::End),
        }
    }

    fn write_frame(&mut self, bytes: &[u8]) -> Result<(), PortError> {
        self.output.push_back(String::from_utf8_lossy(bytes).into_owned());
        Ok(())
    }

    fn memory(&mut self) -> Option<&mut MemoryPort> {
        Some(self)
    }

    fn memory_ref(&self) -> Option<&MemoryPort> {
        Some(self)
    }
}

#[cfg(unix)]
pub use stdio::StdioPort;

/// Newline-framed standard input and output, for the blocking runtimes.
///
/// At open the protocol's standard output moves to a private descriptor and
/// descriptor 1 points at standard error, so stray output from the robot's
/// own code goes to the log instead of breaking the protocol. A runtime that
/// sets O_NONBLOCK on the pipes (libuv does) is waited for with poll.
#[cfg(unix)]
mod stdio {
    use super::{Frame, Port, PortError};
    use std::fs::File;
    use std::io::{ErrorKind, Read, Write};
    use std::mem::ManuallyDrop;
    use std::os::fd::FromRawFd;

    #[repr(C)]
    struct PollFd {
        fd: i32,
        events: i16,
        revents: i16,
    }

    #[cfg(target_os = "linux")]
    type Nfds = core::ffi::c_ulong;
    #[cfg(not(target_os = "linux"))]
    type Nfds = core::ffi::c_uint;

    const POLLIN: i16 = 1;
    const POLLOUT: i16 = 4;

    extern "C" {
        fn dup(fd: i32) -> i32;
        fn dup2(from: i32, to: i32) -> i32;
        fn poll(fds: *mut PollFd, count: Nfds, timeout: i32) -> i32;
    }

    fn wait(fd: i32, events: i16) {
        let mut entry = PollFd { fd, events, revents: 0 };
        // SAFETY: one valid pollfd; an interrupted or failed poll just
        // retries the read or write.
        unsafe { poll(&mut entry, 1, -1) };
    }

    pub struct StdioPort {
        input: ManuallyDrop<File>,
        output: ManuallyDrop<File>,
        output_fd: i32,
        pending: Vec<u8>,
        start: usize,
        ended: bool,
    }

    impl StdioPort {
        pub fn open() -> StdioPort {
            // SAFETY: dup and dup2 on the process's standard descriptors.
            let private = unsafe { dup(1) };
            let output_fd = if private >= 0 && unsafe { dup2(2, 1) } >= 0 { private } else { 1 };
            // SAFETY: descriptors 0 and output_fd stay open for the
            // process; ManuallyDrop never closes them.
            let (input, output) = unsafe { (File::from_raw_fd(0), File::from_raw_fd(output_fd)) };
            StdioPort { input: ManuallyDrop::new(input), output: ManuallyDrop::new(output), output_fd, pending: Vec::new(), start: 0, ended: false }
        }
    }

    impl Port for StdioPort {
        fn read_frame(&mut self, buf: &mut Vec<u8>) -> Result<Frame, PortError> {
            loop {
                if let Some(at) = self.pending[self.start..].iter().position(|byte| *byte == b'\n') {
                    buf.clear();
                    buf.extend_from_slice(&self.pending[self.start..self.start + at]);
                    self.start += at + 1;
                    if buf.last() == Some(&b'\r') {
                        buf.pop();
                    }
                    return Ok(Frame::Data);
                }
                if self.ended {
                    if self.start < self.pending.len() {
                        buf.clear();
                        buf.extend_from_slice(&self.pending[self.start..]);
                        self.start = self.pending.len();
                        return Ok(Frame::Data);
                    }
                    return Ok(Frame::End);
                }
                self.pending.drain(..self.start);
                self.start = 0;
                let mut chunk = [0u8; 65536];
                match self.input.read(&mut chunk) {
                    Ok(0) => self.ended = true,
                    Ok(n) => self.pending.extend_from_slice(&chunk[..n]),
                    Err(error) if error.kind() == ErrorKind::WouldBlock => wait(0, POLLIN),
                    Err(error) if error.kind() == ErrorKind::Interrupted => {}
                    Err(error) => return Err(PortError(error.to_string())),
                }
            }
        }

        fn write_frame(&mut self, bytes: &[u8]) -> Result<(), PortError> {
            let mut line = Vec::with_capacity(bytes.len() + 1);
            line.extend_from_slice(bytes);
            line.push(b'\n');
            let mut written = 0;
            while written < line.len() {
                match self.output.write(&line[written..]) {
                    Ok(0) => return Err(PortError("standard output is closed".into())),
                    Ok(n) => written += n,
                    Err(error) if error.kind() == ErrorKind::WouldBlock => wait(self.output_fd, POLLOUT),
                    Err(error) if error.kind() == ErrorKind::Interrupted => {}
                    Err(error) => return Err(PortError(error.to_string())),
                }
            }
            Ok(())
        }
    }
}

#[cfg(not(unix))]
pub use plain::StdioPort;

/// Standard input and output through std, where there are no descriptors
/// to move (Windows, and wasm32, which has no standard input).
#[cfg(not(unix))]
mod plain {
    use super::{Frame, Port, PortError};
    use std::io::{BufRead, Write};

    pub struct StdioPort;

    impl StdioPort {
        pub fn open() -> StdioPort {
            StdioPort
        }
    }

    impl Port for StdioPort {
        fn read_frame(&mut self, buf: &mut Vec<u8>) -> Result<Frame, PortError> {
            buf.clear();
            match std::io::stdin().lock().read_until(b'\n', buf) {
                Ok(0) => Ok(Frame::End),
                Ok(_) => {
                    while matches!(buf.last(), Some(b'\n' | b'\r')) {
                        buf.pop();
                    }
                    Ok(Frame::Data)
                }
                Err(error) => Err(PortError(error.to_string())),
            }
        }

        fn write_frame(&mut self, bytes: &[u8]) -> Result<(), PortError> {
            let mut out = std::io::stdout().lock();
            out.write_all(bytes).and_then(|_| out.write_all(b"\n")).and_then(|_| out.flush()).map_err(|error| PortError(error.to_string()))
        }
    }
}
