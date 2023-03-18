fn incr_vec_i32(v: Vec<i32>) -> impl Fn() -> Vec<i32> {
    move || {
        let mut res = vec![];
        for x in &v {
            res.push(x + 1);
        }
        return res;
    }
}

fn main() {
    let v = vec![1, 2, 3];

    let incr = incr_vec_i32(v);

    assert_eq!(
        incr(),
        vec![2, 3, 4]
    );
}