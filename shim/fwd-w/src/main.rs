#![windows_subsystem = "windows"]

use std::env;
use std::fs;
use std::io;
use std::os::windows::io::AsRawHandle;
use std::process::{Command, Stdio};

mod ffi;
use ffi::*;

fn main() -> io::Result<()> {
    // Resolve target executable first
    let resolved = {
        let exe = env::current_exe()?;
        let mut target = fs::read_link(&exe)?;
        target.set_file_name(exe.file_name().unwrap());
        fs::read_link(&target)?
    };

    // Returns `Stdio::null()` if the handle is null or unknown, otherwise `Stdio::inherit()`
    fn stdio_handle(stream: &impl AsRawHandle) -> Stdio {
        let handle = stream.as_raw_handle();
        if handle.is_null() || unsafe { GetFileType(handle) } == FILE_TYPE_UNKNOWN {
            Stdio::null()
        } else {
            Stdio::inherit()
        }
    }

    // Launch child process with duplicated stdio
    let _child = Command::new(&resolved)
        .args(env::args().skip(1))
        .stdin(stdio_handle(&std::io::stdin()))
        .stdout(stdio_handle(&std::io::stdout()))
        .stderr(stdio_handle(&std::io::stderr()))
        .spawn()?;

    Ok(())
}
