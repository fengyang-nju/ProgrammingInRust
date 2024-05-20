fn main() {
    let v1 = vec![11, 22, 33];
    let v2 = v1;
    // println!("{}", v1.len()); error[E0382]: borrow of moved value: `v1`
}