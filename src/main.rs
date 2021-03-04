extern crate sdl2;

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::surface::Surface;
use std::time::Duration;

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let video_subsystem = sdl_ctx.video().unwrap();

    let window = video_subsystem.window("Game", 512, 448)
        .position_centered()
        .resizable()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    //let mut player = Entity::new(0, 0, std::path::Path::new("/home/catsanddo/code/senior-project/game/target/debug/player.bmp"));
    let mut player = Entity::new(0, 0, std::path::Path::new("./art/player.bmp"));

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_ctx.event_pump().unwrap();
    'running: loop {
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => player.y -= 3,
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => player.y += 3,
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => player.x -= 3,
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => player.x += 3,
                _ => {},
            }
        }

        let mut s_buffer = Surface::new(256, 224, PixelFormatEnum::RGB24).unwrap();
        player.draw(&mut s_buffer);

        let texture_creator = canvas.texture_creator();
        let texture = s_buffer.as_texture(&texture_creator).unwrap();
        canvas.copy(&texture, None, None).expect("Could not render to the screen");
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

struct Entity<'a> {
    x: i32,
    y: i32,
    sprite: Surface<'a>,
}

impl<'a> Entity<'a> {
    fn new(x: i32, y: i32, path: &std::path::Path) -> Self {
        let image = Surface::load_bmp(path).unwrap();
        Self {
            x: x,
            y: y,
            sprite: image,
        }
    }

    fn draw(&self, surface: &mut Surface<'_>) {
        let dest = sdl2::rect::Rect::new(self.x, self.y, 0, 0);
        self.sprite.blit(None, surface, dest).expect("Could not blit to screen buffer");
    }
}
