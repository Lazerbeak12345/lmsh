use combine::easy::Errors;
use combine::stream::easy::{ParseError,Stream};
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
    use combine::stream::easy::{ParseError,Stream};
    use combine::{EasyParser,many,many1,none_of,Parser,Stream as StreamTrait};
    #[derive(Debug)]
    pub struct Comment(String);
    #[derive(Debug)]
    pub struct CommentBlock(Vec<Comment>);
    #[derive(Debug)]
    pub struct Word(String);
    #[derive(Debug)]
    pub struct Function{
        name:Word,
        statements:Vec<Statement>
    }
    #[derive(Debug)]
    pub enum Statement{
        CommentBlock(CommentBlock),
        Function(Function)
    }
    fn comment<Input>()->impl Parser<Input,Output=Comment>where Input:StreamTrait<Token=char>{
        char('#')
            .with(take_until(char('\n')))
            .map(|string|
                 Comment(string))
    }
    fn comment_block<Input>()->impl Parser<Input,Output=CommentBlock>where Input:StreamTrait<Token=char>{
        many1(comment().skip(char('\n')))
            .skip(many::<Vec<_>,_,_>(char('\n')))
            .map(|comments:Vec<Comment>|
                 CommentBlock(comments))
    }
    fn word<Input>()->impl Parser<Input,Output=Word>where Input:StreamTrait<Token=char>{
        many1(none_of(vec!['$','`','(',' ','\t',';']))
            .map(|string|
                 Word(string))
    }
    pub fn parse<'a>(str:&'a str)->Result<(Vec<Statement>,&'a str),ParseError<Stream<&'a str>>>{
        let function=word()
            .skip(many::<Vec<_>,_,_>(char(' '))
                  .with(char('('))
                  .with(many::<Vec<_>,_,_>(char(' ')))
                  .with(char(')'))
                  .with(many::<Vec<_>,_,_>(char(' ')))
                  .with(char('{'))
                  .with(char('\n')))
            .map(|a|
                 Function{
                     name:a,
                     statements:vec![]
                 });
        let statement=comment_block()
            .map(|comment_block|
                 Statement::CommentBlock(comment_block))
            .or(function.map(|function|
                             Statement::Function(function)));
        let mut statements=many(statement);
        statements.easy_parse(str)//TODO return something else, keeping the call to translate_position in here
    }
}
use tree::*;
fn eval<'a>(tree:Result<(Vec<Statement>,&'a str),ParseError<Stream<&'a str>>>,str:&'a str){
    match tree{
        Ok(tree)=>todo!("Run the code! {:?}",tree),
        Err(Errors{
            position,
            errors
        })=>todo!("Handle error at position {:?} in the source\nMessages: {:?}",position.translate_position(str),errors)
    }
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
    let str=&read(source)?;
    Ok(eval(parse(str),str))
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
