use std::fs::File;
use std::io::Read;

const W0: [&str; 8] = ["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"];
const W1: [&str; 8] = ["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];
const MOV: u8 = 0b10001000;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let mut bin = File::open(filename).unwrap();
    let mut buffer = Vec::new();
    bin.read_to_end(&mut buffer).unwrap();
    println!("bits 16\n");
    buffer.chunks(2).for_each(|ins| {
        let first_byte = ins[0];
        let mask = 0b11111100;
        let d = first_byte & 0b00000010;
        // dbg!("d: {:01b}", d);
        let w = first_byte & 0b00000001;
        // dbg!("w: {:01b}", w);
        let second_byte = ins[1];
        let _modt = (second_byte & 0b11000000) >> 6;
        // dbg!("mod: {:02b}", modt);
        let reg = (second_byte & 0b00111000) >> 3;
        // dbg!("reg: {:03b}", reg);
        let rm = second_byte & 0b00000111;
        // dbg!("rm: {:03b}", rm);
        let order = if d == 0 { (rm, reg) } else { (reg, rm) };
        let reg_w = if w == 0 { W0 } else { W1 };

        match first_byte & mask {
            MOV => {
                println!(
                    "mov {}, {}",
                    reg_w[order.0 as usize], reg_w[order.1 as usize]
                );
            }
            _ => {
                println!("<Unknown Instruction>");
            }
        }
    });
}
