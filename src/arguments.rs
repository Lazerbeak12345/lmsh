use std::env;
pub struct Arguments {
    pub interactive: bool,
    pub version: bool,
    pub login: bool,
}
impl Arguments {
    pub fn parse() -> Result<Arguments, String> {
        let mut arguments = Arguments {
            interactive: true, //TODO "If the -i option is present, or if there are no operands and the shell's standard input and standard error are attached to a terminal, the shell is considered to be _interactive_": TDLR; detect if stdio and stderr are both a terminal
            version: false,
            login: false,
        };
        enum NextDataArg {
            None,
            Caller,
        }
        let mut next_data_arg = NextDataArg::Caller;
        for arg in env::args() {
            let mut arg_chars = arg.chars();
            match arg_chars.next() {
                Some('-') => match arg_chars.next() {
                    Some('-') => match arg.as_str() {
                        "--interactive" => arguments.interactive = true,
                        "--login" => arguments.login = true,
                        "--version" => arguments.version = true,
                        "--" => {} //The standard is to ignore this
                        str => return Err(format!("Unknown keyword argument \"{}\"", str)),
                    },
                    Some(first) => {
                        let mut working_char = Some(first);
                        loop {
                            match working_char{
                                Some('i')=>arguments.interactive=true,
                                Some('l')=>arguments.login=true,
                                Some('V')=>arguments.version=true,
                                Some('c')=>todo!("command_string operand"),
                                Some('s')=>todo!("treat standard input as file (\"unless there are no operands and the -c option is not specified the -s option shall be assumed\")"),
                                Some('a'|'b'|'C'|'e'|'f'|'m'|'n'|'o'|'v'|'x')=>todo!("treat working_char like arg to set"),
                                Some(char)=>return Err(format!("Unknown short argument \"-{}\"",char)),
                                None=>break
                            }
                            working_char = arg_chars.next()
                        }
                    }
                    None => {} //The standard is to ignore this
                },
                Some('+') => todo!(
                    "Some flags must accept a + instead of a -, doing the opposite from the - flag"
                ),
                //TODO support command file argument
                Some(..) => match next_data_arg {
                    NextDataArg::Caller => next_data_arg = NextDataArg::None,
                    NextDataArg::None => return Err(format!("Expected flag, got \"{}\"", arg)),
                },
                None => panic!("The argument should never be empty!"),
            }
        }
        Ok(arguments)
    }
}
