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
    use combine::parser::char::{char,digit,string};
    use combine::parser::repeat::take_until;
    use combine::stream::easy::{ParseError,Stream};
    use combine::{attempt,EasyParser,many,many1,none_of,optional,Parser,Stream as StreamTrait};
    #[derive(Debug)]
    pub struct Function{
        name:String,
        statements:Vec<Statement>
    }
    ///Docstrings for this enum copied from
    ///https://www.tutorialspoint.com/unix/unix-shell-substitutions.htm
    #[derive(Debug)]
    pub enum Substitution{
        /// ```
        /// ${var}
        /// ```
        /// Subtitute the value of `var`.
        Variable(String),
        /// ```
        /// ${var:-word}
        /// ```
        /// If `var` is null or unset, `word` is subtituted for `var`. The value of `var` does not
        /// change.
        VariableAElseB(String,String),
        /// ```
        /// ${var:=word}
        /// ```
        /// If `var` is null or unset, `var` is set to the value of `word`.
        VariableAElseBSet(String,String),
        /// ```
        /// ${var:?message}
        /// ```
        /// If `var` is null or unset, `message` is printed to standard error. This checks that
        /// variables are set correctly.
        VariableAElseMessage(String,String),
        /// ```
        /// ${var:+word}
        /// ```
        /// If `var` is set, `word` is subtituted for `var`. The value of `var` does not change.
        NotVariableAElseB(String,String)
    }
    #[derive(Debug)]
    pub enum Expansion{
        Statement(Statement),
        Substitution(Substitution),
    }
    #[derive(Debug)]
    pub enum StringEscapes{
        Backslash,//\\
        Alert,//\a
        Backspace,//\b
        SuppressTrailingNewline,//\c
        FormFeed,//\f
        NewLine,//\n
        CarriageReturn,//\r
        HorizontalTab,//\t
        VerticalTab//\v
    }
    #[derive(Debug)]
    pub enum ArgumentPart{
        Text(String),
        Escape(StringEscapes),
        DollarExpansion(Expansion),
        TildeExpansion(Statement),
        DoubleQuote(Argument),
        SingleQuote(Argument),
    }
    pub type Argument=Vec<ArgumentPart>;
    #[derive(Debug)]
    pub struct Case{
        argument:Argument,
        parts:Vec<(Argument,Vec<Statement>)>
    }
    #[derive(Debug)]
    pub struct Command{
        program:Argument,
        arguments:Vec<Argument>
    }
    #[derive(Debug)]
    pub enum If{
        If{
            condition:Command,
            statements:Vec<Statement>,
            next:Option<Box<If>>
        },
        Else{
            statements:Vec<Statement>
        }
    }
    #[derive(Debug)]
    pub struct Variable{
        name:String,
        content:Vec<Argument>
    }
    #[derive(Debug)]
    pub enum Statement{
        CommentBlock(String),
        Function(Function),
        Case(Case),
        If(If),
        Command(Command),
        Variable(Variable)
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
    fn variable_name<Input>()->impl Parser<Input,Output=String>where Input:StreamTrait<Token=char>{
        many1(none_of(vec!['$','\'','"','`','(',')',' ','\t',';','}','\\','\n','=',':']))
    }
    fn word<Input>()->impl Parser<Input,Output=String>where Input:StreamTrait<Token=char>{
        many1(none_of(vec!['$','\'','"','`','(',')',' ','\t',';','}','\\','\n']))
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
            .with(many(choice!(many1(none_of(vec!['"','$','`']))
                               .map(|text:String|
                                    ArgumentPart::Text(text)),
                               dollar_expansion()
                               .map(|dollar_expansion|
                                    ArgumentPart::DollarExpansion(dollar_expansion)))))
            .skip(char('"'))
    }
    fn subtitution<Input>()->impl Parser<Input,Output=Substitution>where Input:StreamTrait<Token=char>{
        choice!(digit()
                .map(|digit|
                     Substitution::Variable(String::from(digit))),
                choice!(char('{')
                        .with(variable_name())
                        .skip(char('}')),
                        variable_name())
                .map(|word|
                     Substitution::Variable(word)))
    }
    fn dollar_expansion<Input>()->impl Parser<Input,Output=Expansion>where Input:StreamTrait<Token=char>{
        char('$')
            .with(choice!(subtitution()
                          .map(|subtitution|
                               Expansion::Substitution(subtitution))))
    }
    fn argument<Input>()->impl Parser<Input,Output=Argument>where Input:StreamTrait<Token=char>{
        many1(choice!(word()
                      .map(|word|
                           ArgumentPart::Text(word)),
                      doublequote()
                      .map(|doublequote|
                           ArgumentPart::DoubleQuote(doublequote)),
                      dollar_expansion()
                      .map(|dollar_expansion|
                           ArgumentPart::DollarExpansion(dollar_expansion))))
    }
    fn variable<Input>()->impl Parser<Input,Output=Variable>where Input:StreamTrait<Token=char>{
        variable_name()
            .skip(char('='))
            .and(command())
            .map(|(word,mut command)|{
                let mut content=vec![command.program];
                content.append(&mut command.arguments);
                Variable{
                    name:word,
                    content
                }
            })
    }
    fn space_or_tab<Input>()->impl Parser<Input,Output=char>where Input:StreamTrait<Token=char>{
        char(' ')
            .or(char('\t'))
    }
    fn command<Input>()->impl Parser<Input,Output=Command>where Input:StreamTrait<Token=char>{
        argument()
            .and(many(attempt(many::<String,_,_>(space_or_tab())
                              .with(argument()))))
            .skip(many::<String,_,_>(space_or_tab()))
            .skip(char(';')
                  .or(char('\n')))
            .map(|(program,arguments)|
                 Command{
                     program,
                     arguments
                 })
    }
    fn parse_elif_else<Input>()->impl Parser<Input,Output=Box<If>>where Input:StreamTrait<Token=char>{
        choice!(string("elif").map(|_|todo!("parse elif block")),
                string("else").map(|_|todo!("parse else block")))
    }
    fn parse_if<Input>()->impl Parser<Input,Output=If>where Input:StreamTrait<Token=char>{
        string("if")
            .skip(space_or_tab())
            .with(command())
            .skip(many::<String,_,_>(space_or_tab()))
            .skip(string("then")
                  .with(char('\n')))
            .and(statements())
            .and(optional(parse_elif_else()))
            .skip(string("fi"))
            .map(|((command,statements),next)|
                 If::If{
                     condition:command,
                     statements,
                     next
                 })
    }
    fn case<Input>()->impl Parser<Input,Output=Case>where Input:StreamTrait<Token=char>{
        let many_space_tab_or_nl=||
            many::<String,_,_>(choice!(char(' '),
                                       char('\t'),
                                       char('\n')));
        let case_block_ender=||
            many_space_tab_or_nl()
            .with(string(";;"))
            .skip(many_space_tab_or_nl());
        string("case")
            .skip(space_or_tab())
            .with(argument()
                  .message("case requires an argument"))
            .skip(space_or_tab())
            .skip(string("in")
                  .message("case requires the in keyword"))
            .skip(char(';')
                  .or(char('\n'))
                  .message("case requires a newline or semicolon after the in keyword"))
            .skip(many::<String,_,_>(space_or_tab()))
            .and(many1(argument()
                       .message("case requries a pattern to match")
                       .skip(char(')')
                             .skip(char('\n'))
                             .message("case requires an end-parenthasis"))
                       .and(choice!(attempt(case_block_ender())
                                    .map(|_|
                                         vec![]),//It's supposed to be able to ignore a lack of statements, but this hack should work.
                                    statements()
                                    .message("Must be a valid statement, or none at all.")
                                    .skip(case_block_ender())))))
            .skip(string("esac")
                  .message("case must be closed with an esac"))
            .map(|(argument,parts)|
                 Case{
                     argument,
                     parts
                 })
    }
    fn statement<Input>()->impl Parser<Input,Output=Statement>where Input:StreamTrait<Token=char>{
        many::<Vec<_>,_,_>(char(' '))
            .with(choice!(comment_block()
                          .message("Must be valid comment block")
                          .map(|comment_block|
                               Statement::CommentBlock(comment_block)),
                          case()
                          .message("Must be valid case block")
                          .map(|case|
                               Statement::Case(case)),
                          parse_if()
                          .message("Must be valid if block")
                          .map(|parse_if|
                               Statement::If(parse_if)),
                          attempt(variable())
                          .message("Must be valid variable definition")//TODO does this message even display?
                          .map(|variable|
                               Statement::Variable(variable)),
                          attempt(command())
                          .message("Must be valid command")
                          .map(|command|
                               Statement::Command(command)),
                          function()
                          .message("Must be valid function definition")
                          .map(|function|
                               Statement::Function(function))))
            .message("A statement must be a comment, case, if, variable, or function")
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
