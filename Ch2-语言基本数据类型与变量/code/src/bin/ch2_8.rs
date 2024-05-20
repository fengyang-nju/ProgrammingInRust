fn main() {
    // The value of mutable variables can be changed.
    let mut _x = 1;
    _x = 2; // ok

    // The value of immutable variables cannot be changed.
    let _y = _x;
    // _y = _x; // not ok
}
