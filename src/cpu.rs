use crate::bus::Bus;
use rand::random;
use std::io::{Result, Read};
use std::fs::File;

pub struct Cpu {
    v: [u8; 16],
    i: u16, // used to store memory addresses
    delay_timer: u8,
    pc: u16, // program counter (store currently executing address)
    stack: [u16; 16],
    sp: u8, // the stack pointer (SP) can be 8-bit, it is used to point to the topmost level of the stack
    paused: bool,
    speed: i32,
    pub bus: Bus,
}

impl Cpu {
    pub fn new() -> Self {
       let cpu = Cpu {
            bus: Bus::new(),
            v: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
            paused: false,
            speed: 10,
            delay_timer: 0,
        };
        // cpu.bus.
        cpu
    }

    pub fn load_rom(&mut self, rom: [u8; 3583]) -> Result<()> {
        let start_addr = 0x200;
        for (i, byt) in rom.iter().enumerate() {
            self.bus.write_to_ram((start_addr + i) as u16, *byt);
        }
        Ok(())
    }


    fn get_opcode(&mut self) -> u16 {
        (((self.bus.read_from_ram(self.pc) as u16 ) << 8_u16 ) | (self.bus.read_from_ram(self.pc + 1) as u16))
    }

    pub fn cycle(&mut self) {
        // println!("{}", self.bus.ram);
        for _ in 0..self.speed {
            if !self.paused {
                let opcode = self.get_opcode();
                println!("{:#06X}", opcode);
                self.execute_instruction(opcode);

            }
        }

        if !self.paused {
            // updated timers
        }

        // self.speaker.play_sound(sub_system);
        // // render
        // self.renderer.render();
    }

    pub fn update_timers(&mut self)  {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        // if self.sound_timer > 0 {
        //     self.sound_timer -=1;
        // }
    }

    fn execute_instruction (&mut self, opcode: u16) {
        self.pc += 2;
        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let n = ((opcode & 0x000F) >> 0) as u8;
        let nnn = (opcode & 0x0FFF);
        let kk = (opcode & 0x00FF) as u8;

        match (c, x, y, n) {
            (  0,   0,   0,   0) => { },
            (  0,   0, 0xE,   0) => self.bus.clear_screen(), //
            (  0,   0, 0xE, 0xE) => self.ret(), // Return from subroutine
            (0x1,   _,   _,   _) => { self.pc = nnn; return; } // Jump to location nnn do not increment pc
            (0x2,   _,   _,   _) => self.call(nnn), // call subroutine at nnn
            (0x3,   _,   _,   _) => { if self.read_reg_v(x) == kk { self.pc += 2; }}, // skip next instruction if Vx = kk
            (0x4,   _,   _,   _) => { if self.read_reg_v(x) != kk { self.pc += 2; }}, // skip next instruction if Vx != kk
            (0x5,   _,   _,   0) => { if self.read_reg_v(x) == self.read_reg_v(y) { self.pc += 2; }}, // skip next instruction if Vx = Vy
            (0x6,   _,   _,   _) => self.write_reg_v(x, kk), // set Vx = kk
            (0x7,   _,   _,   _) => self.add_vx_kk(x, kk), // Set Vx = Vx + kk
            (0x8,   _,   _, 0x0) => self.ld_vx_vy(x, y), // Set Vx = Vy
            (0x8,   _,   _, 0x1) => self.or_xy(x, y), // Set Vx = Vx OR Vy
            (0x8,   _,   _, 0x2) => self.and_xy(x, y), // Set Vx = Vx AND Vy
            (0x8,   _,   _, 0x3) => self.xor_xy(x, y), // Set Vx = Vx XOR Vy
            (0x8,   _,   _, 0x4) => self.add_xy(x, y), // Set Vx = Vx + Vy, set VF = carry
            (0x8,   _,   _, 0x5) => self.sub_xy(x, y), // Set Vx = Vx - Vy, set VF = NOT borrow
            (0x8,   _,   _, 0x6) => self.shr_x(x), // Set Vx = Vx SHR 1
            (0x8,   _,   _, 0x7) => self.subn_xy(x, y), // Set Vx = Vy - Vx, set VF = NOT borrow
            (0x8,   _,   _, 0xE) => self.shl_x(x), // Set Vx = Vx SHL 1.
            (0x9,   _,   _,   0) => { if self.read_reg_v(x) != self.read_reg_v(y) { self.pc += 2; }}, // Skip next instruction if Vx != Vy.
            (0xA,   _,   _,   _) => { self.i = nnn }, // Set I = nnn.
            (0xB,   _,   _,   _) => { self.pc = nnn + self.read_reg_v(0) as u16  }, // Jump to location nnn + V0.
            (0xC,   _,   _,   _) => self.write_reg_v(x, random::<u8>() & kk), // Set Vx = random byte AND kk.
            (0xD,   _,   _,   _) => self.drw_x_y(x, y, n), // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
            (0xE,   _, 0x9, 0xE) => {}, // Skip next instruction if key with the value of Vx is pressed.
            (0xE,   _, 0xA, 0x1) => {}, // Skip next instruction if key with the value of Vx is not pressed.
            (0xF,   _, 0x0, 0x7) => self.write_reg_v(x, self.delay_timer), // Set Vx = delay timer value.
            (0xF,   _, 0x0, 0xA) => {}, // Wait for a key press, store the value of the key in Vx.
            (0xF,   _, 0x1, 0x5) => { self.delay_timer = self.read_reg_v(x) }, // Set delay timer = Vx.
            (0xF,   _, 0x1, 0x8) => {}, // Set sound timer = Vx.
            (0xF,   _, 0x1, 0xE) => { self.i = self.i + self.read_reg_v(x) as u16; }, // Set I = I + Vx.
            (0xF,   _, 0x2, 0x9) => {
                self.i = (self.read_reg_v(x) * 5) as u16;
            }, // Set I = location of sprite for digit Vx.
            (0xF,   _, 0x3, 0x3) => {
                let vx = self.read_reg_v(x);
                self.bus.write_to_ram(self.i, vx/100);
                self.bus.write_to_ram(self.i + 1, (vx % 100) / 10);
                self.bus.write_to_ram(self.i + 2, vx % 10);
            }, // Store BCD representation of Vx in memory locations I, I+1, and I+2.
            (0xF,   _, 0x5, 0x5) => self.ld_i_vx(x), // Store registers V0 through Vx in memory starting at location I.
            (0xF,   _, 0x6, 0x5) => self.ld_vx_i(x), // Read registers V0 through Vx from memory starting at location I.
            // _                    => todo!("opcode: {:#04x}", opcode),
            _                    => {},
        }

    }

    fn drw_x_y(&mut self, x:u8, y:u8, n:u8){
        let x = x % 64;
        let y = y % 32;

        self.write_reg_v(0xF, 0);

        for row in 0..n {
            let mut sprite_row = self.bus.read_from_ram(self.i + row as u16);

            for col in 0..8 {
                if (sprite_row & 0x80) > 0 {
                    let vx = self.read_reg_v(x);
                    let vy = self.read_reg_v(y);
                    if self.bus.screen.set_pixel((vx + col) as u32, (vy + row) as u32) {
                      self.write_reg_v(0xF, 1);
                    }
                }
                sprite_row <<= 1;
            }
        }
    }
    fn  ld_vx_vy (&mut self, x:u8, y: u8) {
        let val= self.read_reg_v(y);
        self.write_reg_v(x, val);
    }
    fn ld_i_vx(&mut self, x: u8) {
        for i in 0..x {
            let byt = self.read_reg_v(i);
            self.bus.write_to_ram(self.i, byt);
        }
    }
    fn ld_vx_i(&mut self, x: u8) {
        for i in 0..x {
            let byt = self.bus.read_from_ram(self.i + i as u16);
            self.write_reg_v(i, byt);
        }
    }
    fn shl_x(&mut self, x: u8){
        let arg = self.read_reg_v(x);
        if (( arg & 0x80) >> 7) == 1 {
            self.write_reg_v(0xF, 1);
        } else {
            self.write_reg_v(0xF, 0);
        }

        self.write_reg_v(x, arg << 1);
    }

    fn subn_xy (&mut self, x: u8, y: u8) {
        let arg_1 = self.read_reg_v(x);
        let arg_2 = self.read_reg_v(y);
        let res = arg_2.overflowing_sub(arg_1);

        if arg_2 > arg_1 {
            self.write_reg_v(0xF, 1);
        } else {
            self.write_reg_v(0xF, 0);
        }

        self.write_reg_v(x, res.0);
    }
    fn shr_x (&mut self, x: u8) {
        let arg = self.read_reg_v(x);
        self.write_reg_v(0xF, arg & 0x01);

        // self.write_reg_v(x, arg / 2);
        self.write_reg_v(x, arg >> 1);
    }
    fn and_xy (&mut self, x: u8, y: u8) {
        let arg1 = self.read_reg_v(x);
        let arg2 = self.read_reg_v(y);
        self.write_reg_v(x, arg1 & arg2);
    }
    fn xor_xy (&mut self, x: u8, y: u8) {
        let arg1 = self.read_reg_v(x);
        let byt = self.read_reg_v(y);
        self.write_reg_v(x, arg1 ^ byt);
    }

    fn or_xy (&mut self, x: u8, y: u8) {
        let arg1 = self.read_reg_v(x);
        let byt = self.read_reg_v(y);
        self.write_reg_v(x, arg1 | byt);
    }
    fn add_vx_kk (&mut self, x: u8, kk: u8) {
        let val = self.read_reg_v(x);
        // println!("{} {}",val, kk);
        let res = val.overflowing_add(kk);
        self.write_reg_v(x, res.0);
    }

    fn add_xy (&mut self, x: u8, y: u8) {
        let arg_1 = self.read_reg_v(x);
        let arg_2 = self.read_reg_v(y);

        let (val, overflow_detected) = arg_1.overflowing_add(arg_2);
        self.write_reg_v(x, val);

        if overflow_detected {
            self.write_reg_v(0xF, 1);
        } else {
            self.write_reg_v(0xF, 0);
        }
    }
    fn sub_xy (&mut self, x: u8, y: u8) {
        let arg_1 = self.read_reg_v(x);
        let arg_2 = self.read_reg_v(y);

        let res = arg_1.overflowing_sub(arg_2);

        if arg_1 > arg_2 && res.1 {
            self.write_reg_v(0xF, 1);
        } else {
            self.write_reg_v(0xF, 0);
        }

        self.write_reg_v(x, res.0);
    }

    //noinspection ALL
    fn call(&mut self, addr: u16) {
        if self.sp as usize > self.stack.len() {
            panic!("stack overflow");
        }

        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = addr;
    }

    fn ret (&mut self) {
        if self.sp == 0 {
            panic!("stack underflow");
        }

        self.sp -= 1;
        self.pc = self.stack[self.sp as usize] as u16;
    }

    pub fn read_reg_v(&mut self, index: u8) -> u8 {
        self.v[index as usize]
    }
    pub fn write_reg_v(&mut self, index: u8, value: u8) {
        self.v[index as usize] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cpu = Cpu::new();
        // cpu.load_program()
    }
}