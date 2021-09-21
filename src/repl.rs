#[derive(Debug, Clone, Copy)]
pub enum ReplSource{
    User,
    File,
}
fn read(source:ReplSource,last_errors:ReplError)->Option<String>{
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
pub fn repl(source:ReplSource)->Result<(),ReplError>{
    let mut repl_err=ReplError::ErrorCodes(Vec::new());
    loop{
        match read(source,repl_err){
            Some(data)=>{
                repl_err=eval(data);
                match source{
                    ReplSource::File=>{
                        match &repl_err{
                            ReplError::ErrorCodes(codes)=>if codes.is_empty(){
                                continue
                            },
                            ReplError::SyntaxError(..)=>{}
                        }
                        return Err(repl_err)
                    }
                    //The user will see the error on the next call to read, if there is one.
                    ReplSource::User=>{}
                }
            },
            None=>return Ok(())
        }
    }
}
