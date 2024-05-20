fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}

// error[E0597]: `vec` does not live long enough
// fn main() {
//     let s;
//     {
//         let vec = [7, 3, 5, 9, 2, 4, 1, 5];
//         s = smallest(&vec);
//     }
//     assert_eq!(*s, 1);
// }

fn main() {
    let s;
    let vec = [7, 3, 5, 9, 2, 4, 1, 5];
    s = smallest(&vec);
    assert_eq!(*s, 1);
}