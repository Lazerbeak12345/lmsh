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
                    None=>return Err("Argument was empty!".to_string())//dead code
                }
            }
            Ok(arguments)
        }
    }
}
