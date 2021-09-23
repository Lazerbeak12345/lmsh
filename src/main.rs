use std::process::exit;
use std::env;
use std::path;
use lmsh::arguments::Arguments;
use lmsh::repl::{repl,ReplSource,ReplError,ReplReturn};
fn greet(){
    println!("Welcome to Lazerbeak12345's Micro Shell!");
}
fn get_config_file()->Option<path::PathBuf>{
    let home_config_vec=match env::var_os("HOME"){
        Some(val)=>{
            let mut pb=path::PathBuf::from(val);
            pb.push(".config");
            vec![pb]
        },
        None=>Vec::new()
    };
    match env::var_os("XDG_CONFIG_DIRS"){
        Some(val)=>{
            let mut vec=home_config_vec;
            for path in env::split_paths(&val){
                vec.push(path)
            }
            vec
        },
        None=>home_config_vec
    }.iter().map(|path|{
        let mut new_path=path.clone();
        new_path.push("lmsh");
        new_path.push("init.lmsh");
        new_path
    }).filter(|path|path.exists()).next()//We want to run only the first file.
}
fn run_config_file()->Option<ReplReturn>{
    get_config_file().and_then(|config_file|
                               Some(repl(ReplSource::File{
                                   source:config_file
                               })))
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
        match run_config_file(){
            Some(Ok(()))=>{},
            Some(Err((file,ReplError::ErrorCodes(codes))))=>{
                eprintln!(
                    "During execution of config script these error codes were raised. {:?} {:?}",
                    codes,
                    file
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
