const POWER: u8 = 40;
const MODERATE_TEMPERATURE: u8 = 150;

fn main() {
    let mut temperature = 20;

    'cook: loop {
        temperature += POWER;
        if temperature >= MODERATE_TEMPERATURE {
            loop {
                if temperature == MODERATE_TEMPERATURE {
                    break 'cook;
                }
                temperature -= 1;
            }
        }
    }

    assert!(temperature == MODERATE_TEMPERATURE);
}
