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
    use combine::parser::char::{char,string};
    use combine::parser::repeat::take_until;
    use combine::stream::easy::{ParseError,Stream};
    use combine::{EasyParser,many,many1,none_of,Parser,Stream as StreamTrait};
    #[derive(Debug)]
    pub struct Function{
        name:String,
        statements:Vec<Statement>
    }
    #[derive(Debug)]
    pub enum ArgumentPart{
        Text(String),
        Escape(char),
        DollarExpansion(Statement),
        TildeExpansion(Statement),
        DoubleQuote(Argument),
        SingleQuote(Argument),
    }
    pub type Argument=Vec<ArgumentPart>;
    #[derive(Debug)]
    pub struct Case{
        argument:Argument,
        parts:Vec<(Argument,Vec<Statement>)>,
        default:Option<Vec<Statement>>
    }
    #[derive(Debug)]
    pub enum Statement{
        CommentBlock(String),
        Function(Function),
        Case(Case)
    }
    fn comment_block<Input>()->impl Parser<Input,Output=String>where Input:StreamTrait<Token=char>{
        many1(char('#')
              .with(take_until(char('\n')))
              .and(char('\n'))
              .map(|(mut left,right):(String,char)|{
                  left.push(right);
                  left
              }))
            .skip(many::<Vec<_>,_,_>(char('\n')))
    }
    fn word<Input>()->impl Parser<Input,Output=String>where Input:StreamTrait<Token=char>{
        many1(none_of(vec!['$','\'','"','`','(',')',' ','\t',';']))
    }
    fn function<Input>()->impl Parser<Input,Output=Function>where Input:StreamTrait<Token=char>{
        word()
            .skip(many::<Vec<_>,_,_>(char(' '))
                  .with(char('('))
                  .with(many::<Vec<_>,_,_>(char(' ')))
                  .with(char(')'))
                  .with(many::<Vec<_>,_,_>(char(' ')))
                  .with(char('{'))
                  .with(char('\n')))
            .and(statements())
            .skip(char('}')
                  .with(char('\n')))
            .map(|(word,statements)|
                 Function{
                     name:word,
                     statements
                 })
    }
    fn doublequote<Input>()->impl Parser<Input,Output=Argument>where Input:StreamTrait<Token=char>{
        char('"')
            .with(many(none_of(vec!['"','$','`'])))
            .map(|text|
                 vec![ArgumentPart::Text(text)])
    }
    fn argument<Input>()->impl Parser<Input,Output=Argument>where Input:StreamTrait<Token=char>{
        many(choice!(doublequote()
                     .map(|doublequote|
                          ArgumentPart::DoubleQuote(doublequote))
                     ))
    }
    fn case<Input>()->impl Parser<Input,Output=Case>where Input:StreamTrait<Token=char>{
        string("case")
            .with(char(' ')
                  .or(char('\t')))
            .with(argument())
            .map(|argument|
                 Case{
                     argument,
                     parts:vec![],
                     default:None
                 })
    }
    fn statement<Input>()->impl Parser<Input,Output=Statement>where Input:StreamTrait<Token=char>{
        many::<Vec<_>,_,_>(char(' '))
            .with(choice!(comment_block()
                              .map(|comment_block|
                                   Statement::CommentBlock(comment_block)),
                          case()
                            .map(|case|
                                 Statement::Case(case)),
                          function()
                              .map(|function|
                                   Statement::Function(function))))
    }
    parser!{
        fn statements[Input]()(Input)->Vec<Statement>where[Input:StreamTrait<Token=char>]{
            many(statement())
        }
    }
    pub fn parse<'a>(str:&'a str)->Result<(Vec<Statement>,&'a str),ParseError<Stream<&'a str>>>{
        //TODO return something else, keeping the call to translate_position in here
        statements().easy_parse(str)
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
