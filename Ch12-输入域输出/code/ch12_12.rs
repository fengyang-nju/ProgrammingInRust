use std::path::PathBuf;

fn main() {
    let mut path = PathBuf::new();
    path.push("/home/user");
    path.push("file.txt");

    println!("{}", path.display());
}
