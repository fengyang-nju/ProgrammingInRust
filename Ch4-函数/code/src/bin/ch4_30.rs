fn map_on_vec_i32(v: Vec<i32>, op: impl Fn(i32) -> i32) -> Vec<i32> {
    let mut res = vec![];
    for x in v {
        res.push(op(x));
    }
    return res;
}

fn main() {
    let v = vec![1, 2, 3];

    let v = map_on_vec_i32(v, |x| {x + 1});

    assert_eq!(v, vec![2, 3, 4]);
}