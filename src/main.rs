use std::process::exit;
use std::env;
use std::path;
use lmsh::arguments::Arguments;
use lmsh::repl::{repl, ReplSource};
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
fn run_config_file(){
    match get_config_file(){
        Some(config_file)=>todo!("repl config file {:?}",config_file),
        None=>{}
    }
}
fn main(){
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
