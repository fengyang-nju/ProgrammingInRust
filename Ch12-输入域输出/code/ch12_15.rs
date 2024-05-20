use std::fs;

fn main(){
    fs::create_dir("example_dir").unwrap(); 
    fs::remove_dir("example_dir").unwrap();
}
