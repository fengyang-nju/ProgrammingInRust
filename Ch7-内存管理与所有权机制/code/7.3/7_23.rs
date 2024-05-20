struct Person {
    first_name: String,
    last_name: String,
}

fn main() {
    let p = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
    };
    let rp = &p;
    assert_eq!(rp.first_name, "John");
    assert_eq!((*rp).last_name, "Doe");
}