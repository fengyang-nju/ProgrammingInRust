const POWER: u8 = 40;
const MODERATE_TEMPERATURE: u8 = 150;

fn main() {
    let mut temperature = 20;

    loop {
        temperature += POWER;
        if temperature < MODERATE_TEMPERATURE - POWER {
            continue;
        } else {
            break;
        }
    }

    assert!(temperature <= MODERATE_TEMPERATURE);
}
