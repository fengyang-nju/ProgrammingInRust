struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 2, y: 4 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;
    assert_eq!(rrr.y, 4);
}