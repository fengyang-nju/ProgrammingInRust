fn main(){
    let mut a = String::from("Hello,");
    let b = String::from("world");
    a += &b;
    println!("{:?}", a);
}