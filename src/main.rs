use std::process;
use lmsh::arguments::Arguments;
fn greet(){
    println!("Welcome to Lazerbeak12345's Micro Shell!");
}
#[derive(Debug, Clone, Copy)]
enum ReplSource{
    User,
    File,
}
fn read(source:ReplSource,last_errors:ReplError)->Result<String,()>{
    todo!("Get the code that needs to run! {:?} {:?}", source, last_errors)
}
fn eval(string:String)->ReplError{
    todo!("Run the code! {}",string)
}
#[derive(Debug)]
enum ReplError{
    ErrorCodes(Vec<i32>),
    SyntaxError(String)
}
fn repl(source:ReplSource)->Result<(),ReplError>{
    let mut repl_err=ReplError::ErrorCodes(Vec::new());
    loop{
        match read(source,repl_err){
            Ok(data)=>{
                repl_err=eval(data);
                match source{
                    ReplSource::File=>{
                        match &repl_err{
                            ReplError::ErrorCodes(codes)=>if codes.is_empty(){
                                continue
                            },
                            ReplError::SyntaxError(..)=>{}
                        }
                        return Err(repl_err)
                    }
                    //The user will see the error on the next call to read, if there is one.
                    ReplSource::User=>{}
                }
            },
            Err(..)=>return Ok(())
        }
    }
}
fn run_config_file(){
    todo!("find config file and repl it")
}
fn main() {
    let args=Arguments::parse().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
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
