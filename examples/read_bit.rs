use memacc::bitman::ReadBit;

fn main() {
    let value = 0b1010_0110u8;
    for index in 0..=7 {
        let state = value.read_bit(index);
        println!("State at index {index} is {state}.");
    }
}
