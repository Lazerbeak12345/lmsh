use std::process;
use lmsh::arguments::Arguments;
fn greet(){
    println!("Welcome to Lazerbeak12345's Micro Shell!");
}
fn main() {
    let args=Arguments::parse().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });
    //TODO Run config file (uses non-interactive repl under hood)
    if args.interactive{
        greet();
        //TODO interactive repl
    }
    //TODO on exit return last exit status of ran command
}
