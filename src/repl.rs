mod source{
    use std::path::PathBuf;
    use std::io::Bytes;
    #[derive(Debug)]
    pub enum ReplSource{
        User,
        File(PathBuf)
    }
    pub fn read(source:ReplSource)->Bytes<u8>{
        todo!("Get code from the file {:?}",source)
    }
    pub fn prompt()->Bytes<u8>{
        todo!("Prompt the user for code!")
    }
}
pub use source::ReplSource;
use source::*;
mod tokens{
    //TODO use a parse library
    use std::io::Bytes;
    pub enum ReplToken{}
    #[derive(Debug)]
    pub struct ReplTokens;
    impl ReplTokens{
        pub fn tokenize(bytes:Bytes<u8>)->ReplTokens{
            todo!("convert code stream into token stream{:?}",bytes)
        }
    }
    impl Iterator for ReplTokens{
        type Item = ReplToken;
        fn next(&mut self) -> Option<Self::Item> {
            todo!("get the next token, if possible")
        }
    }
}
use tokens::*;
mod tree{
    //TODO use a token library
    use super::tokens::ReplTokens;
    #[derive(Debug)]
    pub struct ReplTree;
    impl ReplTree{
        pub fn parse(tokens:ReplTokens)->ReplTree{
            todo!("convert token stream into parse tree{:?}",tokens)
        }
    }
}
use tree::*;
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
        ReplSource::File(..)=>eval(ReplTree::parse(ReplTokens::tokenize(read(source)))),
        ReplSource::User=>loop{
            match eval(ReplTree::parse(ReplTokens::tokenize(prompt()))){
                Err(err)=>todo!("Handle error {:?}",err),
                _=>{}
            }
        }
    }
}
