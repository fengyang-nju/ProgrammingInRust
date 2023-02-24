use std::io::{Error, Write};
use std::fs::{OpenOptions};
fn create_file_write_string(st:&str)->Result<usize, Error>{
    let file_result = OpenOptions::new()
                            .write(true).create(true)
                            .open("ch11_8.txt");
    let mut file = match file_result{
        Ok(file)=>{
            file
        },
        Err(open_err) => return Err(open_err)
    };
    match file.write_fmt(format_args!("{}", st)){
        Ok(_) => Ok(st.len()),
        Err(write_err) => Err(write_err)
    }
}