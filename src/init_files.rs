use crate::repl::{repl, ReplSource};
use std::env::{split_paths, var_os};
use std::io::Error;
use std::path::PathBuf;
#[test]
fn get_config_file_test(){
    todo!("Mock 'var_os' and 'PathBuf::exists', call 'get_config_file' then assert on the return")
}
fn get_config_file() -> Option<PathBuf> {
    let home_config_vec = match var_os("HOME") {
        Some(val) => {
            let mut pb = PathBuf::from(val);
            pb.push(".config");
            vec![pb]
        }
        None => Vec::new(),
    };
    match var_os("XDG_CONFIG_DIRS") {
        Some(val) => {
            let mut vec = home_config_vec;
            for path in split_paths(&val) {
                vec.push(path)
            }
            vec
        }
        None => home_config_vec,
    }
    .iter()
    .map(|path| {
        let mut new_path = path.clone();
        new_path.push("lmsh");
        new_path.push("init.lmsh");
        new_path
    })
    .filter(|path| path.exists())
    .next() //We want to run only the first file.
}
#[test]
fn run_config_file_test(){
    todo!("Mock 'get_config_file' and 'repl' then assert on the return")
}
fn run_config_file() -> Option<Result<(), Error>> {
    get_config_file().and_then(|config_file| Some(repl(ReplSource::File(config_file))))
}
#[test]
fn run_profile_test(){
    todo!("Mock 'PathBuf::exists' and 'repl' then assert on the return")
}
fn run_profile() -> Option<Result<(), Error>> {
    let usr_profile = PathBuf::from("/etc/profile");
    if usr_profile.exists() {
        Some(repl(ReplSource::File(usr_profile)))
    } else {
        None
    }
}
#[test]
fn run_init_files_test(){
    todo!("Mock 'run_profile' and 'run_config_file' then assert on the return")
}
//TODO give the user a bare-minimum working shell instead of bailing
pub fn run_init_files(login: bool) -> Option<Result<(), Error>> {
    if login {
        match run_profile() {
            Some(Err(err)) => return Some(Err(err)),
            _ => {}
        }
    }
    run_config_file()
}
