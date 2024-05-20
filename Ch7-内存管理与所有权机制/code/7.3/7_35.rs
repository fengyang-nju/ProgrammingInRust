// error[E0106]: missing lifetime specifier
// struct S {
//     r: &i32,
// }

struct S<'a> {
    r: &'a i32,
}

fn main() {
    let s;
    {
        let x = 10;
        s = S { r: &x };
    }
    assert_eq!(*s.r, 10);
}