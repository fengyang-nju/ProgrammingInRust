use std::io::{Error, Write};
use std::fs::{OpenOptions};
fn create_file_write_string(st:&str)->Result<usize, Error>{
    match OpenOptions::new()
                            .write(true).create(true)
                            .open("ch11_8.txt")?
                            .write_fmt(format_args!("{}", st)) match{
                                Ok(_) => Ok(st.len()),
                                Err(write_err) => Err(write_err)
                            }

}