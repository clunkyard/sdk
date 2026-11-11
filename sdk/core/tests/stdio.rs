//! StdioPort in a real process: lines in on standard input, replies out on
//! standard output, with the robot's own stray output sent to standard
//! error and an O_NONBLOCK standard input waited for with poll.
//!
//! The test runs itself as the robot (CY_STDIO_ROBOT=1), so it has its own
//! main and no libtest output on standard output.

#[cfg(all(unix, not(target_family = "wasm")))]
mod unix {
    use clunkyard::generated::ffi::*;
    use clunkyard::generated::*;
    use std::io::{BufRead, BufReader, Write};
    use std::process::{Command, Stdio};

    extern "C" {
        fn fcntl(fd: i32, command: i32, ...) -> i32;
    }
    const F_GETFL: i32 = 3;
    const F_SETFL: i32 = 4;
    #[cfg(target_os = "linux")]
    const O_NONBLOCK: i32 = 0o4000;
    #[cfg(not(target_os = "linux"))]
    const O_NONBLOCK: i32 = 4;

    /// The robot: a non-blocking standard input, as libuv leaves it, and a
    /// stray print each turn.
    pub fn robot() {
        // SAFETY: fcntl on descriptor 0.
        unsafe { fcntl(0, F_SETFL, fcntl(0, F_GETFL) | O_NONBLOCK) };
        let mut robot = std::ptr::null_mut();
        let name = "clunkyard-test";
        assert_eq!(cy_open(API_MAJOR, API_MINOR, name.as_ptr(), name.len() as u32, "2.0.0".as_ptr(), 5, &mut robot), CY_OK);
        loop {
            let mut event = 0;
            assert_eq!(cy_session_next(robot, &mut event), CY_OK);
            if event == Event::End as i32 {
                break;
            }
            println!("a stray print, which must not reach the protocol");
            if event == Event::State as i32 {
                assert_eq!(cy_drive_throttle(robot, 0.5), CY_OK);
            }
            assert_eq!(cy_session_send(robot), CY_OK);
        }
        cy_close(robot);
    }

    pub fn check() {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let lines = std::fs::read_to_string(root.join("protocol/v2/fixtures/3d-zero-g.ndjson")).unwrap();
        let mut child = Command::new(std::env::current_exe().unwrap())
            .env("CY_STDIO_ROBOT", "1")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let mut input = child.stdin.take().unwrap();
        let mut output = BufReader::new(child.stdout.take().unwrap());
        let mut replies = Vec::new();
        for line in lines.trim().split('\n') {
            // A line at a time, waiting for each reply, so the robot finds
            // its non-blocking input empty and polls.
            std::thread::sleep(std::time::Duration::from_millis(20));
            writeln!(input, "{line}").unwrap();
            input.flush().unwrap();
            let mut reply = String::new();
            output.read_line(&mut reply).unwrap();
            replies.push(reply.trim_end().to_string());
        }
        drop(input);
        let status = child.wait_with_output().unwrap();
        assert!(status.status.success(), "the robot failed: {}", String::from_utf8_lossy(&status.stderr));
        assert!(String::from_utf8_lossy(&status.stderr).contains("a stray print"), "stray output goes to standard error");
        assert!(status.stdout.is_empty(), "nothing after the last reply: {:?}", String::from_utf8_lossy(&status.stdout));
        assert_eq!(replies[0], r#"{"v":2,"type":"ready","sdk":{"name":"clunkyard-test","version":"2.0.0"}}"#);
        for reply in &replies[1..] {
            assert!(reply.starts_with(r#"{"v":2,"type":"act","tick":"#) && reply.ends_with(r#""drive":{"throttle":0.5}}"#), "{reply}");
        }
        assert_eq!(replies.len(), lines.trim().split('\n').count());
        println!("stdio: {} replies over a non-blocking standard input, stray output on standard error", replies.len());
    }
}

fn main() {
    #[cfg(all(unix, not(target_family = "wasm")))]
    {
        if std::env::var_os("CY_STDIO_ROBOT").is_some() {
            unix::robot();
        } else {
            unix::check();
        }
    }
}
