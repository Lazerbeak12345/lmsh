use std::process::exit;
use lmsh::arguments::Arguments;
use lmsh::repl::{repl, ReplSource};
fn greet(){
    println!("Welcome to Lazerbeak12345's Micro Shell!");
}
fn run_config_file(){
    todo!("find config file and repl it")
}
fn main() {
    let args=Arguments::parse().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        exit(1)
    });
    if args.version{
        greet();
        println!("version 0.1.0");
    }else{
        run_config_file();
        if args.interactive{
            greet();
            match repl(ReplSource::User){
                Ok(..)=>return,
                Err(err)=>{
                    panic!("The repl should never return an error in user mode. {:?}",err)
                }
            }
        }
    }
}
