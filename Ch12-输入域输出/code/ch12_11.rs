use std::path::Path;

fn main() {
    let path = Path::new("/home/user/file.txt");

    println!("{:?}", path.parent().unwrap()); 
    println!("{:?}", path.file_name().unwrap()); 
    println!("{:?}", path.extension().unwrap()); 
    
    let path2 = path.join("other.txt");
    println!("{}", path2.display()); 
}
