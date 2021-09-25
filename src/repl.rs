use std::path::PathBuf;
use std::io::Bytes;
#[derive(Debug)]
pub enum ReplSource{
    User,
    File(PathBuf)
}
fn read(source:ReplSource)->Bytes<u8>{
    todo!("Get code from the file {:?}",source)
}
fn prompt()->Bytes<u8>{
    todo!("Prompt the user for code!")
}
#[derive(Debug)]
struct ReplTokens;
//TODO impl Iterator for ReplTokens
fn tokenize(bytes:Bytes<u8>)->ReplTokens{
    todo!("convert code stream into token stream{:?}",bytes)
}
#[derive(Debug)]
struct ReplTree;
fn parse(tokens:ReplTokens)->ReplTree{
    todo!("convert token stream into parse tree{:?}",tokens)
}
fn eval(tree:ReplTree)->ReplReturn{
    todo!("Run the code!{:?}",tree);
}
#[derive(Debug)]
pub enum ReplError{
    ErrorCodes(Vec<i32>),
    SyntaxError(String)
}
pub type ReplReturn=Result<(),(ReplSource,ReplError)>;
/**
 * Run-Eval-Print-Loop.
 *
 * When the result is Ok no errors happened during execution.
 * When result is Err, if it's an ReplError::ErrorCodes then it's a Vec of return codes, otherwise
 *    it's a String with the error message.
 */
pub fn repl(source:ReplSource)->ReplReturn{
    match source{
        ReplSource::File(..)=>eval(parse(tokenize(read(source)))),
        ReplSource::User=>loop{
            match eval(parse(tokenize(prompt()))){
                Err(err)=>todo!("Handle error {:?}",err),
                _=>{}
            }
        }
    }
}
