use std::process::exit;
use lmsh::arguments::Arguments;
use lmsh::repl::{repl, ReplSource};
use std::env;
use std::path;
fn greet(){
    println!("Welcome to Lazerbeak12345's Micro Shell!");
}
fn run_config_file(){
    {//find config file
        let home_config_vec=match env::var_os("HOME"){
            Some(val)=>{
                let mut pb=path::PathBuf::from(val);
                pb.push(".config");
                vec![pb]
            },
            None=>Vec::new()
        };
        let xdg_config_dirs=match env::var_os("XDG_CONFIG_DIRS"){
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
            new_path
        });
        todo!("iterate over each item in xdg_data_dirs till init.lmsh is or init.sh is found inside a folder titled \"lmsh\"")
    }
    //TODO repl config file
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
