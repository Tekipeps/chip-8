use std::fmt;

pub struct Ram {
    pub mem: [u8; 4096],
}

impl  Ram {
    pub fn new() -> Self {
        let mut mem = [0; 4096];
        let sprites:[u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];

        for i in 0..sprites.len() {
            mem[i] = sprites[i];
        }

        Ram {
            mem,
        }
    }

    pub fn write_byt(&mut self, addr: u16, val: u8) {
        self.mem[addr as usize] = val;
    }
    pub fn read_byt(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }
}

impl fmt::Display for Ram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();

        for byts in self.mem.chunks(20) {
            for byt in byts {
                str.push_str(format!("{:#06X} ",byt ).as_str());
            }
            str.push_str("\n");
        }
        write!(f, "{}", str)
    }
}