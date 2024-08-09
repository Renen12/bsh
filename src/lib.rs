use concatenator::*;
use std::ffi::OsString;
use std::fs::{self, read_dir, DirEntry};
use std::io::{self, Write};
use std::process::Command;
pub fn exec(path: String, command: String, mut args: Vec<&str>) {
    let mut done = false;
    for bindir in path.split(";") {
        for exec in fs::read_dir(bindir).unwrap() {
            let exec = exec.unwrap().file_name();
            if OsString::from(&command) == exec {
                io::stdout().flush().unwrap();
                match Command::new(&command).args(&args).status() {
                    Ok(_) => (),
                    Err(e) => {
                        println!("Error executing {}: {}", &command, e);
                    }
                }
                done = true;
            }
        }
        if !done {
            println!("{}: command not found", &command);
            break;
        }
    }
}
