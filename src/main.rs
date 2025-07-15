use std::fs::File;
use std::io::Read;

const W0: [&str; 8] = ["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"];
const W1: [&str; 8] = ["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];
const EA: [[Option<&str>; 2]; 8] = [
    // BX + SI
    [Some(W1[3]), Some(W1[6])],
    // BX + DI
    [Some(W1[3]), Some(W1[7])],
    // BP + SI
    [Some(W1[5]), Some(W1[6])],
    // BP + DI
    [Some(W1[5]), Some(W1[7])],
    // SI
    [Some(W1[6]), None],
    // DI
    [Some(W1[7]), None],
    // DIRECT ACCESS
    [None, None],
    // BX
    [Some(W1[3]), None],
];
const REGISTER_MEMORY_TO_FROM_REGISTER: u8 = 0b10001000;
const IMMEDATE_TO_REGISTER: u8 = 0b10110000;

pub struct X86Decoder {
    source: Vec<u8>,
    start: usize,
    current: usize,
}

impl X86Decoder {
    pub fn new(source: Vec<u8>) -> Self {
        X86Decoder {
            source,
            start: 0,
            current: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.source[self.current - 1]
    }

    pub fn decode(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_opcode();
        }
    }

    fn scan_opcode(&mut self) {
        let opcode = self.advance();
        let mask_6bit = 0b11111100;
        let mask_4bit = 0b11110000;
        // println!("current {}", self.current);
        if opcode & mask_6bit == REGISTER_MEMORY_TO_FROM_REGISTER {
            self.register_memory_to_from_register(opcode);
        } else if opcode & mask_4bit == IMMEDATE_TO_REGISTER {
            self.immediate_to_register(opcode);
        }
    }

    fn register_memory_to_from_register(&mut self, opcode: u8) {
        let d = (opcode & 0b00000010) >> 1;
        // dbg!(d);
        // dbg!("d: {:01b}", d);
        let w = opcode & 0b00000001;
        // dbg!("w: {:01b}", w);
        let second_byte = self.advance();
        let modt = (second_byte & 0b11000000) >> 6;
        let reg = (second_byte & 0b00111000) >> 3;
        // dbg!("reg: {:03b}", reg);
        let rm = second_byte & 0b00000111;
        let reg_w = if w == 0 { W0 } else { W1 };
        if modt == 0 {
            // Memory mode, no displacement*
            if rm != 6 {
                let first = EA[rm as usize][0].unwrap_or("?");
                let second = EA[rm as usize][1].unwrap_or("");
                if d == 1 {
                    println!("mov {}, [{} + {}]", reg_w[reg as usize], first, second);
                } else {
                    println!("mov [{} + {}], {} ", first, second, reg_w[reg as usize]);
                }
            } else {
                println!("<Direct address>")
            }
        }
        if modt == 1 {
            // Memory mode, 8 bit displacement
            if rm != 6 {
                let disp = self.advance();
                let first = EA[rm as usize][0].unwrap_or("?");
                let second = EA[rm as usize][1].unwrap_or("");
                let num = i8::from_le_bytes([disp]);
                println!(
                    "mov {}, [{} + {} + {}]",
                    reg_w[reg as usize], first, second, num
                );
            } else if d == 1 {
                println!("mov {}, [bp]", reg_w[reg as usize],)
            } else {
                println!("mov [bp], {}", reg_w[reg as usize],)
            }
        }
        if modt == 2 {
            // memory mode, 16
            if rm != 6 {
                let disp = self.advance();
                let disp2 = self.advance();
                let first = EA[rm as usize][0].unwrap_or("?");
                let second = EA[rm as usize][1].unwrap_or("");
                println!(
                    "mov {}, [{} + {} + {}]",
                    reg_w[reg as usize],
                    first,
                    second,
                    i16::from_le_bytes([disp, disp2])
                );
            } else if d == 1 {
                println!("mov {}, [bp]", reg_w[reg as usize],)
            } else {
                println!("mov [bp], {}", reg_w[reg as usize],)
            }
        }
        if modt == 3 {
            println!("mov {}, {}", reg_w[rm as usize], reg_w[reg as usize]);
        }
    }

    fn immediate_to_register(&mut self, opcode: u8) {
        let w = (opcode & 0b00001000) >> 3;
        // dbg!(w);
        let reg = opcode & 0b00000111;
        // dbg!(reg);
        if w == 0 {
            let imm = self.advance();
            let num = i8::from_le_bytes([imm]);
            println!("mov {}, {}", W0[reg as usize], num);
        } else if w == 1 {
            let imm = self.advance();
            let imm2 = self.advance();
            let num = i16::from_le_bytes([imm, imm2]);
            println!("mov {}, {}", W1[reg as usize], num);
        }
    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let mut bin = File::open(filename).unwrap();
    let mut buffer = Vec::new();
    bin.read_to_end(&mut buffer).unwrap();
    println!("bits 16\n");
    let mut decoder = X86Decoder::new(buffer);
    decoder.decode();
}
