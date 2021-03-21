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
const SCALE: i32 = 2;

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let _sdl_image_ctx = sdl2::image::init(sdl2::image::InitFlag::PNG).unwrap();
    let video_subsystem = sdl_ctx.video().unwrap();

    let window = video_subsystem.window("Game", (WIDTH * SCALE) as u32, (HEIGHT * SCALE) as u32)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    // New clock with a max FPS of 60
    let mut clock = util::Clock::new(60);

    // Initialize player and walls, then center camera on player
    let (mut player, mut walls) = load_scene("./level.json");
    let mut camera_pos = [player.sx - WIDTH / 2 + 4, player.sy - HEIGHT / 2 + 6];

    // Initial clear of screen
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();
    canvas.set_scale(SCALE as f32, SCALE as f32).expect("Could not set scale");

    let mut event_pump = sdl_ctx.event_pump().unwrap();

    // Mainly a test
    let mut x_key: i8 = 0;

    // Main loop
    'running: loop {
        // NOTE: Consolidate event loops
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
                    let (p, w) = load_scene("./level.json");
                    player = p;
                    walls = w;
                    //camera_pos[0] = 0;
                    //camera_pos[1] = 0;
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

        // Clear canvas
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // Jump code
        if x_key == 1 && player.jump {
            player.mv(0, -278);
            player.jump = false;
        } else if x_key == 3 && player.vy < 0 {
            player.vy = -50;
        }

        // Draw walls
        for wall in &walls {
            wall.draw(&mut canvas, &camera_pos[..]);
        }

        // Draw & update player
        player.draw(&mut canvas, &camera_pos[..]);
        player.update(clock.delta_time(), &mut walls);

        // Center camera on player
        camera_pos[0] = camera_pos[0] + (((player.sx - WIDTH / 2 + 4) - camera_pos[0]) as f32 * 0.05) as i32;
        // Lerp the y if the player is on ground or near the top/bottom of the screen
        if player.jump || player.sy - camera_pos[1] > HEIGHT - 50 || player.sy - camera_pos[1] < 50 {
            camera_pos[1] = camera_pos[1] + (((player.sy - HEIGHT / 2 + 6) - camera_pos[1]) as f32 * 0.05) as i32;
        }

        // Present canvas & advance game clock
        canvas.present();
        clock.tick();
    }
}

// Load and return scene data
fn load_scene(file_name: &str) -> (entity::Player<'static>, Vec<entity::Wall<'static>>) {
    // Load in the raw json from file
    let raw_data = std::fs::read_to_string(file_name).unwrap();
    // Parse json data
    let scene_data = json::parse(&raw_data).unwrap();

    // Create player based on player start position
    let player = entity::Player::new(scene_data["player"]["x"].as_f32().unwrap() as i32, scene_data["player"]["y"].as_f32().unwrap() as i32,
        std::path::Path::new("./art/player.png"));

    // Load walls into a Vec
    let mut walls = Vec::new();
    for wall in scene_data["walls"].members() {
        walls.push(entity::Wall::new(wall["x"].as_f32().unwrap() as i32, wall["y"].as_f32().unwrap() as i32, true, std::path::Path::new("./art/brick.png")));
    }

    // Return player and walls
    (player, walls)
}
