use std::fmt::{Display,Formatter,Error};
use std::io;
mod source{
    use std::path::PathBuf;
    use std::io::Result as IOResult;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;
    #[derive(Debug)]
    pub enum ReplSource{
        User,
        File(PathBuf)
    }
    fn prompt()->IOResult<String>{
        todo!("Prompt the user for code!")
    }
    fn open(path:PathBuf)->IOResult<String>{
        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        Ok(contents)
    }
    pub fn read(source:ReplSource)->IOResult<String>{
        match source{
            ReplSource::User=>prompt(),
            ReplSource::File(path)=>open(path)
        }
    }
}
pub use source::ReplSource;
use source::*;
mod tree{
    extern crate combine;
    use combine::parser::char::char;
    use combine::parser::repeat::take_until;
    use combine::error::StringStreamError;
    use combine::{many,Parser,many1};
    #[derive(Debug)]
    pub struct Comment(String);
    #[derive(Debug)]
    pub struct CommentBlock(Vec<Comment>);
    pub fn parse<'a>(string:String)->Result<(Vec<CommentBlock>,String),StringStreamError>{
        let comment=char('#')
            .with(take_until(char('\n')))
            .map(|string:String|Comment(string));
        let comment_block=many1(comment.skip(char('\n')))
            .skip(many::<Vec<_>,_,_>(char('\n')))
            .map(|comments:Vec<Comment>|CommentBlock(comments));
        let mut comment_blocks=many1(comment_block);
        let(nodes,string)=comment_blocks.parse(string.as_str())?;
        Ok((nodes,String::from(string)))
    }
}
use tree::*;
use combine::error::StringStreamError;
fn eval(tree:Result<(Vec<CommentBlock>,String),StringStreamError>){
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
    Ok(eval(parse(read(source)?)))
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
