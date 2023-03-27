fn incr_vec_i32() -> fn(&Vec<i32>) -> Vec<i32> {
    fn f(v: &Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        for x in v {
            res.push(x + 1);
        }
        return res;
    }
    return f;
}

fn main() {
    let v = vec![1, 2, 3];

    let incr = incr_vec_i32();

    assert_eq!(
        incr(&v),
        vec![2, 3, 4]
    );
}