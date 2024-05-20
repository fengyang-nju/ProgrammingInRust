fn main() {
    let s = "Rust Programming Language.";
    let c = s.get(0..4);
    assert_eq!(c, Some("Rust"));

    let c = s.get(0..100);
    assert_eq!(c, None);

    if c.is_some() {
        println!("{}", c.unwrap());
    }
}