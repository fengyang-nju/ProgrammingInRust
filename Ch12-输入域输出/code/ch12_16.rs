use std::fs::File;
use std::io::Write;
fn create_file() -> std::io::Result<File>{
    let mut file = File::create("test.txt")?;
    Ok(file)
}
fn main(){
    let mut file = create_file().unwrap();
    file.write_all(b"Hello, world!").unwrap();
}
