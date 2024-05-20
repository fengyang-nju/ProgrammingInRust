fn div(x: f32, y: f32) -> Result<f32, String> {
    if y == 0. {
        Err(String::from("Divide by zero"))
    } else {
        Ok(x / y)
    }
}

fn main() {
    let res = div(6., 2.);
    println!("{:?}", res);
    let res = div(3., 0.);
    println!("{:?}", res);
}