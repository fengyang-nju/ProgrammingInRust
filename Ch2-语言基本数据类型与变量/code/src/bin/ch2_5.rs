use std::mem::size_of;
fn main() {
    let _unit: () = ();
    // The size of unit type is 0.
    assert_eq!(size_of::<()>(), 0);
}
