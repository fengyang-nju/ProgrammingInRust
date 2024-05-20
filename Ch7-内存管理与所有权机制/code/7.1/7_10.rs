fn main() {
    let t: (char, i8, u32) = ('b', 5, 34);

    let a: i32 = 25;
    let b: &i32 = &a;

    let c: &&i32 = &b;

    let a: [i32; 3] = [55, 66, 77];

    let v: Vec<i32> = vec![55, 66, 77];

    let s1: &[i32] = &a[0..2];
    let s2: &[i32] = &v[1..];

    let s1: String = String::from("hello");

    let s2: &str = "hello";
}

struct Data {
    nums: Vec<usize>,
    dimension: (usize, usize),
}

struct Data(Vec<usize>);

struct Data;

enum HTTPStatus {
    Ok,
    NotFound,
}

enum HTTPStatus {
    Ok = 200,
    NotFound = 404,
}

enum Data {
    Empty,
    Number(i32),
    Array(Vec<i32>),
}

enum Data {
    Empty,
    Number(i32),
    Array(Box<Vec<i32>>), // 使用 Box 代替
}

pub enum Option<T> {
    None,
    Some(T),
}