fn main() {
    let _f = 0.0f32;
    let f = 1.1920929e-7;

    assert_eq!(f, f32::EPSILON);
    assert_eq!(2.220446049250313e-16, f64::EPSILON);

    assert_eq!(-3.4028235e38, f32::MIN);
    assert_eq!(3.4028235e38, f32::MAX);

    assert_eq!(-1.7976931348623157e308, f64::MIN);
    assert_eq!(1.7976931348623157e308, f64::MAX);
}
