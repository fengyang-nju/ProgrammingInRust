fn main() {
    let mut vec = vec![1];
    let item: Option<i32> = vec.pop();
    assert!(item.is_some());
    let item: Option<i32> = vec.pop();
    assert!(item.is_none());
}