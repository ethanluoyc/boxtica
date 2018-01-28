extern crate sdl2;

use sdl2::Sdl;
use sdl2::rect::{Rect};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::Window;
use sdl2::pixels::Color;

const WINDOW_SIZE: u32 = 360;
const BLOCK_SIZE: u32 = 60;

pub struct WorldRenderer {
    canvas: Canvas<Window>
}

impl WorldRenderer {
    pub fn new(sdl_context: &mut Sdl) -> WorldRenderer {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("boxtica", WINDOW_SIZE, WINDOW_SIZE)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        return WorldRenderer { canvas: window.into_canvas().build().unwrap() }
    }

    pub fn draw(&mut self, x: u32, y: u32) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(255, 210, 0));
        self.canvas.fill_rect(Rect::new((x * BLOCK_SIZE) as i32,
                                        (y * BLOCK_SIZE) as i32,
                                        BLOCK_SIZE, BLOCK_SIZE));
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        self.canvas.fill_rect(Rect::new(((x+1) * BLOCK_SIZE) as i32,
                                        ((y+1) * BLOCK_SIZE) as i32,
                                        BLOCK_SIZE, BLOCK_SIZE));
        self.canvas.present();
    }
}
