use std::fs::File;
use crate::renderer::Renderer;
use std::io::{Read, Result};
use crate::chip8::SCALE;
use sdl2::render::WindowCanvas;
use crate::speaker::Speaker;
use sdl2::AudioSubsystem;

const MEM_SIZE: usize = 4096;


#[derive(Clone, Default, Debug)]
pub struct Stack {
    next_index: usize,
    items: [u16; 16],
}

impl Stack {
    /// The size of the stack.
    pub const SIZE: usize = 16;

    /// Creates a new empty stack.
    pub fn new() -> Self {
        Self::default()
    }

    /// Pushes an item to the stack if it is not [full](Self::is_full).
    ///
    /// # Returns
    /// - `true`: if pushed successfully
    /// - `false`: if already full
    pub fn push(&mut self, item: u16) -> bool {
        if self.is_full() {
            false
        } else {
            self.items[self.next_index as usize] = item;
            self.next_index += 1;

            true
        }
    }

    /// Pops an item from the stack if it not [empty](Self::is_empty).
    ///
    /// # Returns
    /// - `Some(u16)`: the popped item
    /// - `None`: if already empty
    pub fn pop(&mut self) -> Option<u16> {
        if self.is_empty() {
            None
        } else {
            self.next_index -= 1;
            let item = self.items[self.next_index as usize];

            Some(item)
        }
    }

    /// Returns whether the stack is full.
    pub fn is_full(&self) -> bool {
        self.next_index >= self.items.len()
    }

    /// Returns whether the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.next_index == 0
    }

    /// Returns the number of items in the stack.
    pub fn num_items(&self) -> usize {
        self.next_index
    }
}

pub struct Cpu<'a> {
    registers: [u8; 16],
    i: u16, // used to store memory addresses
    delay_timer: u8,
    sound_timer: u8,
    // timer register
    // sound register
    pc: u16, // program counter (store currently executing address)
    sp: u8, // the stack pointer (SP) can be 8-bit, it is used to point to the topmost level of the stack
    stack: Stack,
    memory: [u8; MEM_SIZE],
    paused: bool,
    speed: i32,
    pub renderer: Renderer<'a>,
    speaker: Speaker,
}

impl <'a> Cpu <'a> {
    pub fn new(canvas: &'a mut  WindowCanvas) -> Self {
        Cpu {
            renderer: Renderer::new(SCALE, canvas),
            speaker: Speaker::new(),
            registers: [0x00; 16],
            i: 0x0000,
            pc: 0x200,
            sp: 0x00,
            stack: Stack::new(),
            memory: [0x000; MEM_SIZE],
            paused: false,
            speed: 10,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn load_sprite_into_memory(&mut self) {
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
            self.memory[i] = sprites[i];
        }
    }
    pub fn load_from_file(&mut self, filename: String) -> Result<()> {
        let mut file = File::open(filename)?;

        file.read(&mut self.memory[0x200..])?;
        Ok(())
    }
    pub fn get_opcode(&mut self) -> u16 {
        (self.memory[self.pc as usize] as u16) << 8 | (self.memory[(self.pc + 1) as usize] as u16)
    }

    pub fn cycle(&mut self, sub_system: &AudioSubsystem) {
        for _i in 0..self.speed {
            if !self.paused {
                let opcode = self.get_opcode();
                self.execute_instruction(opcode);
            }
        }

        if !self.paused {
            // updated timers
        }

        self.speaker.play_sound(sub_system);
        // render
        self.renderer.render();
    }

    pub fn update_timers(&mut self)  {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -=1;
        }
    }

    pub fn execute_instruction (&mut self, opcode: u16) {
        self.pc += 1;
        let x = (opcode & 0x0F00) >> 8;
        let y = (opcode & 0x00F0) >> 4;

        match opcode & 0xF000 {
            0x000 => match opcode {
                0x00E0 => {
                    self.renderer.clear();
                    return;
                },
                0x00EE => {
                    self.pc = match self.stack.pop() {
                        Some(x) => x,
                        None => self.pc,
                    };
                    return;
                },
                _ => {}
            },
            _ => {}
        }
    }
    pub fn display_file_content(&self) {
        for byte in self.memory {
            print!("0x{:03x} ", byte);
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::cpu::Stack;

    #[test]
    fn test_pop() {
        let mut stack = Stack::new();

        assert_eq!(stack.is_empty(), true);
    }
}