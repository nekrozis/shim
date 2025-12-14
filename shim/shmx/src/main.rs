use std::env;
use std::fs;
use std::io;
use std::process::Command;

fn main() -> io::Result<()> {
    let resolved = {
        let exe = env::current_exe()?;
        let mut target = fs::read_link(&exe)?;
        target.set_file_name(exe.file_name().unwrap());
        fs::read_link(&target)?
    };

    let status = Command::new(&resolved).args(env::args().skip(1)).status()?;

    std::process::exit(status.code().unwrap());
}
