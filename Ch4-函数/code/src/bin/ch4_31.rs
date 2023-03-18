fn main() {
    let v = vec![1, 2, 3];

    let v = v.iter()
            .map(|x| x + 1)
            .collect::<Vec<i32>>();

    assert_eq!(v, vec![2, 3, 4]);
}