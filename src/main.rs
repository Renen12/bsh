use bsh::*;
use concatenator::*;
use ctrlc;
use std::borrow::Borrow;
use std::env::{self};
use std::io;
use std::io::Write;
use std::process::exit;
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
        // refactor this into a modular function
        if !builtin {
            exec(path.to_string(), command.to_string(), args);
        }
    }
}
