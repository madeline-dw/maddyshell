use std::env;
use std::io::{self, stdin,stdout,Write};
use std::path::Path;
use std::process::Command;
use std::str::SplitWhitespace;

fn main() {
    loop {
        let current_dir = env::current_dir().unwrap();
        print!(">  {}", current_dir.display());
        let _ = stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                match change_directory(args.clone()) {
                    Ok(()) => continue,
                    Err(e) => println!("error: {}", e),
                }
            },
           "exit" => return,
            command => {
                let child = Command::new(command)
                    .args(args)
                    .spawn();

                match child {
                    Ok(mut child) => { child.wait(); },
                    Err(e) => eprintln!("error: {}", e),
                }
            }
        }
    }
}

fn change_directory(args: SplitWhitespace) -> io::Result<()>  {
    let new_dir = args.peekable().peek().map_or("/", |x| *x);
    let root = Path::new(new_dir);

    env::set_current_dir(&root)
}

