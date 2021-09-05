extern crate sdl2;
use crate::modules::screen;
use self::sdl2::render::WindowCanvas;
use self::sdl2::pixels::Color;


pub struct Screen<'a> {
    pub buffer: [u8; 32*64],
    pub scale: u32,
    pub canvas: &'a WindowCanvas,
}

pub trait ScreenProps {
      const COLS: usize = 64;
      const ROWS: usize = 32;
}
impl ScreenProps for Screen {}

impl Screen {
    pub fn new(scale: u32, mut canvas: &WindowCanvas) -> Screen {

        Screen {
            buffer: [0x00; (Screen::ROWS*Screen::COLS)],
            scale,
            canvas
        }
    }
    pub fn update_val_in_buf(&mut self, val: u8,  index: usize ) {
        self.buffer[index] = val;
    }

     pub fn set_pixel(&mut self, mut x: usize, mut y: usize) -> bool{
        if x > Screen::COLS {
            x -= Screen::COLS;
        } else if x < 0 {
            x += Screen::COLS;
        }
        if y > Screen::ROWS {
            x -= Screen::COLS;
        } else if x < 0 {
            y += Screen::ROWS;
        }

        self.buffer[x + (y * Screen::COLS)] ^= 1;
        return self.buffer[x + (y * Screen::COLS)] != 1;
    }

    pub fn clear_screen(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.present();


    }

    pub fn paint(&mut self, mut x: usize, mut y: usize) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.set_pixel(x, y);
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

    }
}





