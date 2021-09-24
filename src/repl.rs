use std::path::PathBuf;
#[derive(Debug)]
pub enum ReplSource{
    User,
    File{
        source:PathBuf,
        line:i32,
        char:i32
    },
}
impl ReplSource{
    pub fn new_file(source:PathBuf)->ReplSource{
        ReplSource::File{
            source,
            line:0,
            char:0
        }
    }
}
fn read(source:&ReplSource,last_errors:ReplError)->Option<String>{
    todo!("Get the code that needs to run! {:?} {:?}", source, last_errors)
}
fn eval(string:String)->ReplError{
    todo!("Run the code! {}",string)
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
    let mut repl_err=ReplError::ErrorCodes(Vec::new());
    loop{
        match read(&source,repl_err){
            Some(data)=>{
                repl_err=eval(data);
                match source{
                    ReplSource::File{..}=>return Err((source,repl_err)),
                    ReplSource::User=>match&repl_err{
                        ReplError::ErrorCodes(codes)=>if!codes.is_empty(){
                            println!("Return codes:{:?}",codes)
                        },
                        ReplError::SyntaxError(message)=>{
                            println!("Syntax err:{}",message)
                        }
                    }
                }
            },
            None=>return Ok(())
        }
    }
}