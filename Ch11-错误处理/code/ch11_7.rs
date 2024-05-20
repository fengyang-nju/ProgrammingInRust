use std::fs::OpenOptions;
fn main(){
    let _file= OpenOptions::new()
                        .write(true).create_new(true)
                        .open("ch11_6.txt")
                        .unwrap();
    let _file_dup = OpenOptions::new()
                            .write(true).create_new(true)
                            .open("ch11_6.txt")
                            .expect("File Exists!");
    //thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 17, kind: AlreadyExists, message: "File exists" }', ch11_6.rs:3:80
}
