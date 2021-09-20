use std::env;
//use std::error::Error;
//use std::fs;
use std::process;
struct Arguments{
    interactive:bool,
    //login:bool,
    //help:bool
}
impl Arguments{
    pub fn parse(args:env::Args) -> Result<Arguments, String>{
        let mut arguments=Arguments{
            interactive:false,
            //login:false,
            //help:false
        };
        enum NextDataArg{
            None,
            Caller
        }
        let mut next_data_arg=NextDataArg::Caller;
        for arg in args{
            let mut arg_chars=arg.chars();
            match arg_chars.next(){
                Some('-')=>match arg_chars.next(){
                    Some('-')=>match arg.as_str(){
                        "--interactive"=>arguments.interactive=true,
                        str=>return Err(format!("Unknown keyword argument \"{}\"",str))
                    },
                    Some(first)=>{
                        let mut working_char=Some(first);
                        loop{
                            match working_char{
                                Some('i')=>arguments.interactive=true,
                                Some(char)=>return Err(format!("Unknown short argument \"-{}\"",char)),
                                None=>break
                            }
                            working_char=arg_chars.next()
                        }
                    },
                    None=>return Err("A \"-\" on it's own doesn't work as a flag".to_string())
                },
                Some(..)=>match next_data_arg{
                    NextDataArg::Caller=>next_data_arg=NextDataArg::None,
                    NextDataArg::None=>{
                        return Err(format!("Expected flag, got \"{}\"",arg))
                    }
                },
                None=>return Err("Argument was empty!".to_string())//dead code
            }
        }
        Ok(arguments)
    }
}
fn greet(){
    println!("Welcome to Lazerbeak12345's Micro Shell!");
}
fn main() {
    //TODO handle args
    let args=Arguments::parse(env::args()).unwrap_or_else(|err| {
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
