use std::fs;

fn main(){
    let entries = fs::read_dir(".").unwrap();

    for entry in entries { 
        println!("{}", entry.unwrap().path().display());
    }
}
