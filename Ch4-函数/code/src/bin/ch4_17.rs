fn main() {
    let _ = std::fs::File::create("ProgrammingInRust.txt");

    use std::fs::File;
    let _ = File::create("ProgrammingInRust.txt");
}