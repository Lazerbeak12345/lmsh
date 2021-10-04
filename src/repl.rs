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
    //TODO use a token library
    use std::io::{Bytes,Error};
    use std::fs::File;
    use std::string::FromUtf8Error;
    #[derive(Debug)]
    pub enum ReplToken{
        Literal(String),
        Semicolon
    }
    pub struct ReplTokens{
        bytes:Bytes<File>,
        semicolon:bool
    }
    enum TokenLiteralError{
        IO(Error),
        Unicode(FromUtf8Error)
    }
    impl ReplTokens{
        pub fn tokenize(bytes:Bytes<File>)->ReplTokens{
            ReplTokens{
                bytes,
                semicolon:false
            }
        }
        fn consume_comment(&mut self)->Result<(),Error>{
            loop{
                match self.bytes.next(){
                    None=>return Ok(()),
                    Some(Ok(b'\n'))=>return Ok(()),
                    Some(Ok(..))=>{},
                    Some(Err(err))=>return Err(err)
                }
            }
        }
        fn consume_literal(&mut self,first:u8)->Result<Option<ReplToken>,TokenLiteralError>{
            let mut data=vec![first];
            loop{
                match self.bytes.next(){
                    None|Some(Ok(b'\n'|b' '|b'\t'))=>break,
                    Some(Ok(b';'))=>{
                        self.semicolon=true;
                        break
                    },
                    Some(Err(err))=>return Err(TokenLiteralError::IO(err)),
                    Some(Ok(byte))=>data.push(byte)
                }
            }
            match String::from_utf8(data){
                Ok(data_as_str)=>return Ok(Some(ReplToken::Literal(data_as_str))),
                Err(err)=>return Err(TokenLiteralError::Unicode(err))
            }
        }
    }
    impl Iterator for ReplTokens{
        type Item = ReplToken;
        fn next(&mut self)->Option<Self::Item>{
            loop{ 
                if self.semicolon{
                    self.semicolon=false;
                    return Some(ReplToken::Semicolon)
                }
                match self.bytes.next(){
                    None=>return None,
                    Some(Err(err))=>{
                        eprintln!("Error while reading from file: {}",err);
                        return None
                    },
                    Some(Ok(b'#'))=>match self.consume_comment(){
                        Ok(())=>{},//Just ignore the comment
                        Err(err)=>{
                            eprintln!("Error while reading from file during comment: {}",err);
                            return None
                        }
                    },
                    Some(Ok(b'\n'|b' '|b'\t'))=>{},
                    Some(Ok(quote@(b'\''|b'"')))=>todo!("Handle {}quotes",quote),
                    Some(Ok(b'`'))=>todo!("Handle backticks"),
                    Some(Ok(b'$'))=>todo!("Handle $ escape thingy"),
                    Some(Ok(b';'))=>self.semicolon=true,
                    Some(Ok(byte))=>return match self.consume_literal(byte){
                        Ok(literal)=>literal,
                        Err(TokenLiteralError::IO(err))=>{
                            eprintln!("Error while reading from file during literal: {}",err);
                            None
                        },
                        Err(TokenLiteralError::Unicode(err))=>{
                            eprintln!("Error while converting literal to usable value: {}",err);
                            None
                        }
                    }
                }
            }
        }
    }
}
mod tree{
    //TODO use a parse library
    use super::tokens::ReplTokens;
    use std::io::Bytes;
    use std::fs::File;
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
        pub fn parse(bytes:Bytes<File>)->ReplTree{
            let mut tokens=ReplTokens::tokenize(bytes);
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
    Ok(eval(ReplTree::parse(read(source)?)))
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
