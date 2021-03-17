extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::{Scancode, Keycode};

#[path = "./entities.rs"]
mod entity;
#[path = "./util.rs"]
mod util;

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let _sdl_image_ctx = sdl2::image::init(sdl2::image::InitFlag::PNG).unwrap();
    let video_subsystem = sdl_ctx.video().unwrap();

    let window = video_subsystem.window("Game", 512, 448)
        .position_centered()
        .resizable()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let mut clock = util::Clock::new(60);
    let mut player = entity::Player::new(0, 0, std::path::Path::new("./art/player.png"));

    let mut walls = Vec::new();
    for x in 0..32 {
        for y in 0..20 {
            walls.push(entity::Wall::new(x * 8, y * 8 + 64, true, std::path::Path::new("./art/brick.png")));
        }
    }
    walls.push(entity::Wall::new(16, 56, true, std::path::Path::new("./art/brick.png")));
    walls.push(entity::Wall::new(16, 48, true, std::path::Path::new("./art/brick.png")));
    for x in 0..5 {
        walls.push(entity::Wall::new(56 + x * 8, 40, true, std::path::Path::new("./art/brick.png")));
    }

    let camera = util::Camera;

    canvas.set_draw_color(Color::WHITE);
    canvas.clear();
    canvas.present();
    canvas.set_scale(2.0, 2.0).expect("Could not set scale");
    let mut event_pump = sdl_ctx.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        let keys = sdl2::keyboard::KeyboardState::new(&event_pump);
        let scancodes = keys.pressed_scancodes();
        for key in scancodes {
            match key {
                Scancode::X => {
                    if player.jump {
                        player.mv(0, -278);
                        player.jump = false;
                    }
                },
                Scancode::Left => player.mv(-50, 0),
                Scancode::Right => player.mv(50, 0),
                _ => (),
            }
        }
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                    if !player.attack {
                        player.attack = true;
                        player.frame = 4.0;
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::X), .. } => {
                    if player.vy < 0 {
                        player.vy = -50;
                    }
                },
                //Event::KeyDown { keycode: Some(Keycode::Up), .. } => player.mv(0, -3),
                //Event::KeyDown { keycode: Some(Keycode::Down), .. } => player.mv(0, 3),
                //Event::KeyDown { keycode: Some(Keycode::Left), .. } => player.mv(-300, 0),
                //Event::KeyDown { keycode: Some(Keycode::Right), .. } => player.mv(3, 0),
                _ => {},
            }
        }

        for wall in &walls {
            wall.draw(&mut canvas);
        }

        player.draw(&mut canvas);
        player.update(clock.delta_time(), &mut walls);

        camera.update(&mut player, walls.as_mut_slice());

        canvas.present();
        clock.tick();
    }
}
