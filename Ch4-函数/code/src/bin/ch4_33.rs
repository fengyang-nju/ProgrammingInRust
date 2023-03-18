fn kill() -> ! {
    eprintln!("a < 0");
    panic!("killed in diverging function")
}

fn main() {
    let a = -1;
    if a < 0 {
        kill();
    }
}