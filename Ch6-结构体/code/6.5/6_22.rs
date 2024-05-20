enum Location {
    Africa,
    America,
    Asia,
    Europe,
    Oceania,
}

fn main() {
    let loca = Location::Asia;
    match loca {
        Location::Asia => println!("Asia."),
        _ => (),
    }
}