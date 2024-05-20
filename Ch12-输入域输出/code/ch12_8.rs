fn main(){
    println!("{}", format!("Dec number {0:#x} is equal to Hex number {0}", 20)); //=>Dec number 0x14 is equal to Hex number 20
    println!("{}", format!("Expand precision to 5, {0} => {0:.5}", 0.1)); //=>Expand precision to 5, 0.1 => 0.10000
}