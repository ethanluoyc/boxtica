extern crate sdl2;

// use super::maze::Maze;
use super::block::Block;
use super::maze::Maze;
use super::direction::Direction;

use sdl2::Sdl;
use sdl2::rect::{Rect};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::Window;
use sdl2::pixels::Color;


const WINDOW_SIZE: u32 = 360;
const BLOCK_SIZE: u32 = 60;

pub struct WorldRenderer {
    canvas: Canvas<Window>,
    maze: Maze
}


fn block_to_color(blk: &Block) -> sdl2::pixels::Color {
    match blk {
        &Block::Boxtica => { Color::RGB(95, 201, 231) }
        &Block::Player | &Block::Wall
            | &Block::Target(_) => { Color::RGB(95, 201, 231) }
        &Block::Empty => { Color::RGB(43, 48, 59) }
    }
}

impl WorldRenderer {
    pub fn new(sdl_context: &mut Sdl) -> WorldRenderer {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("Boxtica", WINDOW_SIZE, WINDOW_SIZE)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        return WorldRenderer { canvas: window.into_canvas().build().unwrap(),
                               maze: Maze::new(6, 6)
        }
    }

    pub fn draw(&mut self) {
        self.canvas.set_draw_color(Color::RGB(43, 48, 59));
        self.canvas.clear();
        for i in 0..self.maze.data.len() {
            let x = (i / 6) as u32;
            let y = (i % 6) as u32;
            let blk_ = &self.maze.data[(x * 6 + y) as usize];
            self.canvas.set_draw_color(block_to_color(&blk_));
            self.canvas.fill_rect(Rect::new((x * BLOCK_SIZE) as i32,
                                            (y * BLOCK_SIZE) as i32,
                                            BLOCK_SIZE, BLOCK_SIZE));
        }
        self.canvas.present();

    }

    // pub fn move(&self, d: Direction) {
    //     //      use Direction::*;
    //     //        self.maze.
    // }
}
