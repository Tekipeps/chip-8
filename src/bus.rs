use crate::ram::Ram;
use crate::speaker;
use crate::renderer::Renderer;
use crate::keyboard::{Keyboard};
use sdl2::EventPump;

pub struct Bus {
    pub ram: Ram,
    pub screen: Renderer,
    keyboard: Keyboard,
}

impl Bus {
    pub(crate) fn new() -> Self {
        Bus {
            ram: Ram::new(),
            screen: Renderer::new(),
            keyboard: Keyboard::new()
        }
    }
    pub fn handle_keyboard_event (&mut self, events: &mut EventPump) {
        self.keyboard.handle_keyboard_event(events);
    }
    pub fn write_to_ram(&mut self, address: u16, byte: u8) {
        self.ram.write_byt(address, byte);
    }
    pub fn read_from_ram(&mut self, address: u16) -> u8 {
        self.ram.read_byt(address)
    }
    pub fn draw_byte(&mut self, byt: u8, x:u8, y:u8) -> bool {
        self.screen.draw_byte(byt, x, y)
    }
    pub fn clear_screen (&mut self) {
        self.screen.clear();
    }
}