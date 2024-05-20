// static mut STASH: &i32;
// fn foo(p: &i32) {
//     STASH = p;
// }

// static mut STASH: &i32 = &123;
// fn foo(p: &i32) {
//     unsafe {
//         STASH = p;
//     }
// }

static mut STASH: &i32 = &123;
fn foo(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}