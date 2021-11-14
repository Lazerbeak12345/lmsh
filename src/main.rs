use lmsh::arguments::Arguments;
use lmsh::init_files::run_init_files;
use lmsh::repl::{repl, ReplSource};
use std::process::exit;
fn greet() {
    println!("Welcome to Lazerbeak12345's Micro Shell!");
}
fn main() {
    let args = Arguments::parse().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        exit(1)
    });
    if args.version {
        greet();
        println!("version 0.1.0")
    } else {
        match run_init_files(args.login) {
            Some(Ok(())) => {}
            Some(Err(err)) => {
                eprintln!("In a config file {}", err);
                exit(2)
            }
            None => {}
        };
        if args.interactive {
            greet();
            match repl(ReplSource::User) {
                Ok(()) => return,
                Err(err) => {
                    //The message should be given to the user directly.
                    unreachable!(
                        "The repl should never return an error in user mode. In user mode {}.",
                        err
                    )
                }
            }
        }
    }
}
