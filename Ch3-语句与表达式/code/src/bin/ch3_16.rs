use std::process::exit;

const POWER: u8 = 40;

fn main() {
    let mut _temperature = 20;

    loop {
        _temperature += POWER;
    }
}
