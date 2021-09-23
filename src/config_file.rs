use std::path::PathBuf;
use std::env::{var_os,split_paths};
use crate::repl::{ReplReturn,repl,ReplSource};
fn get_config_file()->Option<PathBuf>{
    let home_config_vec=match var_os("HOME"){
        Some(val)=>{
            let mut pb=PathBuf::from(val);
            pb.push(".config");
            vec![pb]
        },
        None=>Vec::new()
    };
    match var_os("XDG_CONFIG_DIRS"){
        Some(val)=>{
            let mut vec=home_config_vec;
            for path in split_paths(&val){
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
pub fn run_config_file()->Option<ReplReturn>{
    get_config_file().and_then(|config_file|
                               Some(repl(ReplSource::File{
                                   source:config_file
                               })))
}
