fn map_on_vec_i32(v: Vec<i32>, op: fn(i32) -> i32) -> Vec<i32> {
    let mut res = vec![];
    for x in v {
        res.push(op(x));
    }
    return res;
}

fn main() {
    let v = vec![1, 2, 3];

    fn plus_one_i32(x: i32) -> i32 {
        x + 1
    }

    let v = map_on_vec_i32(v, plus_one_i32);
    assert_eq!(v, vec![2, 3, 4]);

    let v = map_on_vec_i32(v, |x| {x + 1});
    assert_eq!(v, vec![3, 4, 5]);
}