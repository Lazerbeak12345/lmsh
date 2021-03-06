use lmsh::arguments::Arguments;
use lmsh::init_files::run_init_files;
use lmsh::repl::{repl, ReplSource};
use std::process::exit;
fn greet() {
    println!(
        "Welcome to Lazerbeak12345's Minimal Shell! {}.\nThis software is under the {} licence.",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_LICENSE")
    );
}
fn main() {
    let args = Arguments::parse().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        exit(1)
    });
    if args.version {
        greet();
        println!("version {}", env!("CARGO_PKG_VERSION"))
    } else {
        if let Some(Err(err)) = run_init_files(args.login) {
            eprintln!("In a config file {}", err);
            exit(2)
        }
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
