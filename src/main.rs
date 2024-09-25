use bsh::*;
use concatenator::*;
use ctrlc;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::env::{self};
use std::io;
use std::io::Write;
use std::process::exit;
fn main() {
    let path = "/usr/bin;/usr/sbin";
    let programargs: Vec<_> = env::args().collect();
    if &programargs.len() > &1 {
        if programargs[1] == "--help" {
            println!(
                "{} -c [Command] or {} with no arguments",
                &programargs[0], &programargs[0]
            );
            exit(1);
        }
        if &programargs[1] == "-c" {
            let programargs: Vec<&str> = programargs.iter().map(|s| &**s).collect();
            exec(
                path.to_string(),
                programargs
                    .get(2)
                    .unwrap_or_else(|| {
                        eprintln!("No command was supplied to -c.");
                        exit(1);
                    })
                    .to_string(),
                programargs.split_at(3).1.into(),
            );
            exit(0);
        }
    }
    match ctrlc::set_handler(move || {
        io::stdout().flush().unwrap();
        println!("");
    }) {
        Ok(_) => (),
        Err(_) => (),
    };
    let mut envvars: HashMap<String, String> = HashMap::new();
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
        if command == "-" {
            let mut exit = false;
            if args.len() < 2 {
                println!("\"-\" - Set a variable. - = [Variable] [Value]");
                exit = true;
            }
            if args.len() < 3 {
                main();
            }
            if exit != true {
                if &args[0].to_string() == "=" {
                    envvars.insert(args[1].to_string(), args[2].to_string());
                } else {
                    println!("\"-\" - Set a variable. - = [Variable] [Value]");
                }
            }
            builtin = true;
            env::set_var(args[1], args[2]);
        }
        if !builtin {
            exec(path.to_string(), command.to_string(), args);
        }
    }
}
