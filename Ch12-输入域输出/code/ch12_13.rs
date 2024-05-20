use std::fs;

fn main() {
    fs::create_dir("example_dir").expect("fails to create example_dir");
}
