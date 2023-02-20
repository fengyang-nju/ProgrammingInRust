use std::fs::File;

fn main() {
    let file_result = File::open("ch11_5.txt");
    let _file = match file_result{
        Ok(file) => file,
        Err(error) => panic!("{}", error),
    };
}