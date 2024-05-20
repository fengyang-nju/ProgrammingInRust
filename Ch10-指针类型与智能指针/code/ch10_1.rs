fn first_word_len(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn main(){
    let mut a = String::from("Rust is the best programming language.");
    let len_ = first_word_len(&a);
    println!("first word = {}", &a[..len_]);

    a.clear(); // a now keeps the value ""
    //as a has been modified, the length of its first word has been invalid afterwards
    //println!("first word = {}", &a[..len_]);
    //error! Index out of bounds
}