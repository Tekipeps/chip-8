use crate::modules::Screen;
use sdl2::Sdl;
use sdl2::render::WindowCanvas;

pub struct VM <'a>{
    rom_length: u16,
    pc: u16,
    I: u16,
    opcode: u16,
    sp: u16,
    v: [u8; 16],
    memory: [u8; 4096],
    pub screen: Screen<'a>,
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    draw_flag: bool,
    wrap_x: String,
    wrap_y: String,
    clock_speed: u16,
    timer_speed: u16,
    screen_buffer: u8,
}

impl VM {
    pub fn new(sdl: &Sdl, canvas: &WindowCanvas ) -> Self {
        VM {
            memory: [0; 4096],
            rom_length: 0,
            pc: 0,
            I: 0,
            opcode: 0,
            sp: 0,
            v: [0; 16],
            screen: Screen::new(15, canvas),
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            draw_flag: false,
            wrap_x: String::new(),
            wrap_y: String::new(),
            clock_speed: 0,
            timer_speed: 0,
            screen_buffer: 0,
        }
    }
    pub fn display_screen(&self)  {
        self.screen.draw()
    }
}