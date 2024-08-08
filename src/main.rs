use concatenator::*;
use ctrlc;
use std::borrow::Borrow;
use std::env::{self};
use std::ffi::OsString;
use std::fs;
use std::io::Write;
use std::process::{exit, Command};
use std::{fs::read_dir, io};
fn main() {
    let path = "/usr/bin;/usr/sbin";
    ctrlc::set_handler(move || {
        io::stdout().flush().unwrap();
        println!("");
    })
    .unwrap();
    let mut current_dir = env::var("HOME").unwrap();
    loop {
        let mut command = String::new();
        let prompt = cat(&current_dir, ">".to_string().borrow());
        env::set_current_dir(&current_dir).unwrap();
        io::stdout().write_all(&prompt.as_bytes()).unwrap();
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut command)
            .expect("Error reading command from standard input");
        let command = command.replace("\n", "");
        let mut args = command.split(' ').collect::<Vec<_>>();
        args.remove(0);
        let command = command.split(' ').collect::<Vec<_>>()[0];
        if command == "" {
            continue;
        }
        //builtin shell commands
        let mut noargstocd = false;
        let mut builtin = false;
        if command == "exit" {
            exit(0)
        }
        if command == "cd" {
            if args.len() == 0 {
                env::set_current_dir(env::var("HOME").unwrap()).unwrap();
                current_dir = env::current_dir().unwrap().display().to_string();
                noargstocd = true;
            }
            if !noargstocd {
                match env::set_current_dir(&args[0]) {
                    Ok(_) => {
                        current_dir = env::current_dir().unwrap().display().to_string();
                    }
                    Err(e) => {
                        eprintln!("Error changing directory: {}", e);
                    }
                }
            }
            builtin = true;
        }
        let mut done = false;
        // refactor this into a modular function
        if !builtin {
            for bindir in path.split(";") {
                for exec in read_dir(bindir).unwrap() {
                    let exec = exec.unwrap().file_name();
                    if OsString::from(&command) == exec {
                        io::stdout().flush().unwrap();
                        match Command::new(&command).args(&args.clone()).status() {
                            Ok(_) => (),
                            Err(e) => {
                                println!("Error executing {}: {}", &command, e);
                            }
                        }
                        done = true;
                    }
                }
            }
            if !done {
                println!("{}: command not found", &command);
            }
        }
    }
}
