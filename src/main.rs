use std::fs::File;
use std::io::Read;

fn main() {
    let mut buffer = Vec::new();
    let mut bin = File::open("listing_0037_single_register_mov").unwrap();
    bin.read_to_end(&mut buffer).unwrap();
    dbg!(buffer.len());
    const w0: [&str; 8] = ["AL", "CL", "DL", "BL", "AH", "CH", "DH", "BH"];
    const w1: [&str; 8] = ["AX", "CX", "DX", "BX", "SP", "BP", "SI", "DI"];
    // mov = 100010 (binary pattern for MOV instruction)

    let first_byte = buffer[0];
    let mask = 0b11111100; // Mask for first 6 bits (binary: 11111100)
    let mov_pattern = 0b10001000; // 100010 shifted left by 2 bits

    if (first_byte & mask) == mov_pattern {
        println!("First 6 bits match MOV pattern: 100010");
        let d = first_byte & 0b00000010;
        println!("d: {:01b}", d);
        let w = first_byte & 0b00000001;
        println!("w: {:01b}", w);
        let second_byte = buffer[1];
        let modt = (second_byte & 0b11000000) >> 6;
        println!("mod: {:02b}", modt);
        let reg = (second_byte & 0b00111000) >> 3;
        println!("reg: {:03b}", reg);
        let rm = (second_byte & 0b00000111);
        println!("rm: {:03b}", rm);
        if w == 0 {
            println!("mov {}, {}", w0[rm as usize], w0[reg as usize]);
        } else {
            println!("mov {}, {}", w1[rm as usize], w1[reg as usize]);
        }
    } else {
        println!("First 6 bits do NOT match MOV pattern");
        println!("Expected: 100010, Got: {:06b}", first_byte >> 2);
    }
}
