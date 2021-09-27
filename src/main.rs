use std::process::exit;
use lmsh::arguments::Arguments;
use lmsh::repl::{repl,ReplSource,ReplError};
use lmsh::init_files::run_init_files;
fn greet(){
    println!("Welcome to Lazerbeak12345's Micro Shell!");
}
fn main(){
    let args=Arguments::parse().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        exit(1)
    });
    if args.version{
        greet();
        println!("version 0.1.0")
    }else{
        //TODO pass true if it's a login shell
        match run_init_files(true){
            Some(Ok(()))=>{},
            Some(Err((file,ReplError::ErrorCodes(codes))))=>{
                eprintln!(
                    "During execution of {:?} these error codes were raised: {:?}",
                    file,
                    codes
                );
                exit(match codes.last(){
                    Some(&code)=>code,
                    None=>{
                        eprintln!("List of codes was empty...");
                        2
                    }
                })
            },
            Some(Err((file,ReplError::SyntaxError(message))))=>{
                eprintln!("Error:\"{}\" at {:?}",message,file);
                exit(3)
            },
            None=>{}
        };
        if args.interactive{
            greet();
            match repl(ReplSource::User){
                Ok(())=>return,
                Err(err)=>{
                    //The message should be given to the user directly.
                    panic!("The repl should never return an error in user mode. {:?}",err)
                }
            }
        }
    }
}
