extern crate sdl2;

use sdl2::surface::Surface;

pub struct Entity<'a> {
    pub x: i32,
    pub y: i32,
    pub sprite: Surface<'a>,
}

impl<'a> Entity<'a> {
    pub fn new(x: i32, y: i32, path: &std::path::Path) -> Self {
        let image = sdl2::image::LoadSurface::from_file(path).unwrap();
        Self {
            x: x,
            y: y,
            sprite: image,
        }
    }

    pub fn draw(&self, surface: &mut Surface<'_>) {
        let dest = sdl2::rect::Rect::new(self.x, self.y, 0, 0);
        self.sprite.blit(None, surface, dest).expect("Could not blit to screen buffer");
    }
}
