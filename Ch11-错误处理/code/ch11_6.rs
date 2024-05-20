use std::fs::OpenOptions;
use std::io::ErrorKind;
fn main(){
    OpenOptions::new
    let _file = match OpenOptions::new().write(true).create_new(true).open("ch11_6.txt"){
        Ok(file) => file,
        Err(err) => panic!("{}", err)
    };
    let _file_dup = match OpenOptions::new().write(true).create_new(true).open("ch11_6.txt"){
        Ok(file) => file,
        //match this
        Err(err) => match err.kind(){
            ErrorKind::AlreadyExists => panic!("File exists!"),
            _ => panic!("{}", err)
        }
    };
}