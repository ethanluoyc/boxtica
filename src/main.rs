extern crate sdl2;

mod push_box;

use push_box::block::Block;
use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use push_box::renderer::WorldRenderer;


pub fn main() {
    let mut sdl_context: Sdl = sdl2::init().unwrap();
    let mut renderer = WorldRenderer::new(&mut sdl_context);
    let mut x: u32 = 0;
    let mut y: u32 = 0;

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    x += 1;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    x -= 1;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    y -= 1;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    y += 1;
                },
                _ => {}
            }
            renderer.draw(&Block::Boxtica);
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
