//TODO split this into two files
pub mod repl{
    #[derive(Debug, Clone, Copy)]
    pub enum ReplSource{
        User,
        File,
    }
    fn read(source:ReplSource,last_errors:ReplError)->Option<String>{
        todo!("Get the code that needs to run! {:?} {:?}", source, last_errors)
    }
    fn eval(string:String)->ReplError{
        todo!("Run the code! {}",string)
    }
    #[derive(Debug)]
    pub enum ReplError{
        ErrorCodes(Vec<i32>),
        SyntaxError(String)
    }
    pub fn repl(source:ReplSource)->Result<(),ReplError>{
        let mut repl_err=ReplError::ErrorCodes(Vec::new());
        loop{
            match read(source,repl_err){
                Some(data)=>{
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
                None=>return Ok(())
            }
        }
    }
}
pub mod arguments{
    use std::env;
    pub struct Arguments{
        pub interactive:bool,
        pub version:bool,
    }
    impl Arguments{
        pub fn parse() -> Result<Arguments, String>{
            let mut arguments=Arguments{
                interactive:false,
                version:false,
            };
            enum NextDataArg{
                None,
                Caller
            }
            let mut next_data_arg=NextDataArg::Caller;
            for arg in env::args(){
                let mut arg_chars=arg.chars();
                match arg_chars.next(){
                    Some('-')=>match arg_chars.next(){
                        Some('-')=>match arg.as_str(){
                            "--interactive"=>arguments.interactive=true,
                            "--version"=>arguments.version=true,
                            str=>return Err(format!("Unknown keyword argument \"{}\"",str))
                        },
                        Some(first)=>{
                            let mut working_char=Some(first);
                            loop{
                                match working_char{
                                    Some('i')=>arguments.interactive=true,
                                    Some('v')=>arguments.version=true,
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
                    None=>panic!("The argument should never be empty!")
                }
            }
            Ok(arguments)
        }
    }
}
