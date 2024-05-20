fn main() {
    let s = vec![1, 2, 3];
    let index = 3;
    if (index >= s.len()){
        panic!("index {} out of bounds!", index);
    }else{
        println!("{}", s[index]);
    }
}

