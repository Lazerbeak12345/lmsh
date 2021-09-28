use std::fmt::{Display,Formatter,Error};
use std::io;
mod source{
    use std::path::PathBuf;
    use std::io::prelude::*;
    use std::io::{Bytes,Error};
    use std::fs::File;
    #[derive(Debug)]
    pub enum ReplSource{
        User,
        File(PathBuf)
    }
    fn prompt()->Result<Bytes<File>,Error>{
        todo!("Prompt the user for code!")
    }
    fn open(path:PathBuf)->Result<Bytes<File>,Error>{
        Ok(File::open(path)?.bytes())
    }
    pub fn read(source:ReplSource)->Result<Bytes<File>,Error>{
        match source{
            ReplSource::User=>prompt(),
            ReplSource::File(path)=>open(path)
        }
    }
}
pub use source::ReplSource;
use source::*;
mod tokens{
    //TODO use a parse library
    use std::io::Bytes;
    use std::fs::File;
    #[derive(Debug)]
    pub enum ReplToken{}
    pub struct ReplTokens{
        bytes:Bytes<File>
    }
    impl ReplTokens{
        pub fn tokenize(bytes:Bytes<File>)->ReplTokens{
            ReplTokens{bytes}
        }
    }
    impl Iterator for ReplTokens{
        type Item = ReplToken;
        fn next(&mut self)->Option<Self::Item>{
            let current_token=None;
            loop{
                match self.bytes.next(){
                    None=>break,
                    Some(Err(err))=>{
                        eprintln!("{}",err);
                        return None
                    },
                    Some(Ok(b'#'))=>todo!("Keep grabbing bytes till it's a newline"),
                    Some(Ok(byte))=>todo!("Handle byte {}",byte)
                }
            }
            current_token
        }
    }
}
use tokens::*;
mod tree{
    //TODO use a token library
    use super::tokens::ReplTokens;
    #[derive(Debug)]
    struct ReplCommand{
        program:String,
        args:Vec<String>
    }
    #[derive(Debug)]
    pub struct ReplTree{
        commands:Vec<ReplCommand>
    }
    impl ReplTree{
        pub fn parse(mut tokens:ReplTokens)->ReplTree{
            let tree=ReplTree{
                commands:Vec::new()
            };
            loop{
                match tokens.next(){
                    Some(token)=>todo!("Match on token type {:?}",token),
                    None=>break
                }
            }
            tree
        }
    }
}
use tree::*;
fn eval(tree:ReplTree){
    todo!("Run the code!{:?}",tree);
}
pub enum ReplError{
    ErrorCodes(Vec<i32>),
    SyntaxError(String)
}
impl Display for ReplError{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(),Error> {
        match self{
            ReplError::ErrorCodes(codes)=>write!(f,"these error codes were raised: {:?}",codes),
            ReplError::SyntaxError(err)=>write!(f,"this error was raised: {}",err)
        }
    }
}
///Like repl but no loop
fn rep(source:ReplSource)->Result<(),io::Error>{
    Ok(eval(ReplTree::parse(ReplTokens::tokenize(read(source)?))))
}
/**
 * Run-Eval-Print-Loop.
 *
 * When the result is Ok no errors happened during execution.
 * When result is Err, if it's an ReplError::ErrorCodes then it's a Vec of return codes, otherwise
 *    it's a String with the error message.
 */
pub fn repl(source:ReplSource)->Result<(),io::Error>{
    match source{
        ReplSource::File(..)=>rep(source),
        ReplSource::User=>loop{
            match rep(ReplSource::User){
                Err(err)=>todo!("Handle error {}",err),
                _=>{}
            }
        }
    }
}
