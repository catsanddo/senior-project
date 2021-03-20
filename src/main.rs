extern crate sdl2;
extern crate json;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::{Scancode, Keycode};

#[path = "./entities.rs"]
mod entity;
#[path = "./util.rs"]
mod util;

const WIDTH: i32 = 256;
const HEIGHT: i32 = 224;

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

    let (mut player, mut walls) = load_scene("./level.json");
    let mut camera_pos = [0, 0];

    canvas.set_draw_color(Color::WHITE);
    canvas.clear();
    canvas.present();
    canvas.set_scale(2.0, 2.0).expect("Could not set scale");
    let mut event_pump = sdl_ctx.event_pump().unwrap();
    let mut x_key: i8 = 0;
    'running: loop {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        let keys = sdl2::keyboard::KeyboardState::new(&event_pump);
        let scancodes = keys.pressed_scancodes();
        if x_key == 3 { x_key = 0; }
        if x_key == 2 { x_key = 3; }
        for key in scancodes {
            match key {
                Scancode::X => {
                    if x_key == 0 { x_key = 1; } else
                    if x_key == 1 { x_key = 2; } else
                    if x_key == 3 { x_key = 2; }

                    /*
                    if player.jump {
                        player.mv(0, -278);
                        player.jump = false;
                    }
                    */
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
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    let (mut p, mut w) = load_scene("./level.json");
                    player = p;
                    walls = w;
                },
                Event::KeyUp { keycode: Some(Keycode::X), .. } => {
                    /*
                    if player.vy < 0 {
                        player.vy = -50;
                    }
                    */
                },
                //Event::KeyDown { keycode: Some(Keycode::Up), .. } => player.mv(0, -3),
                //Event::KeyDown { keycode: Some(Keycode::Down), .. } => player.mv(0, 3),
                //Event::KeyDown { keycode: Some(Keycode::Left), .. } => player.mv(-300, 0),
                //Event::KeyDown { keycode: Some(Keycode::Right), .. } => player.mv(3, 0),
                _ => {},
            }
        }

        if x_key == 1 && player.jump {
            player.mv(0, -278);
            player.jump = false;
        } else if x_key == 3 && player.vy < 0 {
            player.vy = -50;
        }

        for wall in &walls {
            wall.draw(&mut canvas, &camera_pos[..]);
        }

        player.draw(&mut canvas, &camera_pos[..]);
        player.update(clock.delta_time(), &mut walls);

        camera_pos[0] = player.sx - WIDTH / 2;
        camera_pos[1] = player.sy - HEIGHT / 2;

        canvas.present();
        clock.tick();
    }
}

fn load_scene(file_name: &str) -> (entity::Player<'static>, Vec<entity::Wall<'static>>) {
    let raw_data = std::fs::read_to_string(file_name).unwrap();
    let scene_data = json::parse(&raw_data).unwrap();
    let player = entity::Player::new(scene_data["player"]["x"].as_f32().unwrap() as i32, scene_data["player"]["y"].as_f32().unwrap() as i32,
        std::path::Path::new("./art/player.png"));
    let mut walls = Vec::new();
    for wall in scene_data["walls"].members() {
        walls.push(entity::Wall::new(wall["x"].as_f32().unwrap() as i32, wall["y"].as_f32().unwrap() as i32, true, std::path::Path::new("./art/brick.png")));
    }
    (player, walls)
}
