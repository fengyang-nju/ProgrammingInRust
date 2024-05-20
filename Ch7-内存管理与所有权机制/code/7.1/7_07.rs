struct S(i32);

impl Drop for S {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn main() {
    let x = S(1);
    {
        let y = S(2);
    }
    println!("exit inner scope.")
}