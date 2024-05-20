const POWER: u8 = 40;
const MODERATE_TEMPERATURE: u8 = 150;

fn main() {
    let mut temperature = 20;

    while temperature < MODERATE_TEMPERATURE {
        temperature += POWER;
    }

    assert!(temperature >= MODERATE_TEMPERATURE);
}
