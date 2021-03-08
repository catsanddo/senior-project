extern crate sdl2;

use sdl2::surface::Surface;
use sdl2::rect::Rect;

trait TypeInfo {
    fn type_of(&self) -> &'static str;
}

// Player
pub struct Player<'a> {
    sprite: Surface<'a>,
    rect: Rect,
    vx: i32,
    vy: i32,
    flip: bool,
}

impl<'a> Player<'a> {
    pub fn new(x: i32, y: i32, path: &std::path::Path) -> Self {
        let image: Surface = sdl2::image::LoadSurface::from_file(path).unwrap();
        let mut rect = image.rect();
        rect.set_x(x);
        rect.set_y(y);
        Self {
            sprite: image,
            rect: rect,
            vx: 0,
            vy: 0,
            flip: false,
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let texture_creator = canvas.texture_creator();
        let texture = self.sprite.as_texture(&texture_creator).unwrap();
        
        canvas.copy_ex(&texture, None, self.rect.clone(), 0.0, None, self.flip, false).expect("Could not render player");
    }

    pub fn update(&mut self, walls: &[Wall]) {
        // Animation
        if self.vx < 0 {
            self.flip = true;
        } else if self.vx > 0 {
            self.flip = false;
        }

        // Gravity
        //self.vy += 2;

        // Movement
        if self.collide(self.rect.x() + self.vx, self.rect.y(), walls) {
            while !(self.collide(self.rect.x() + self.vx.signum(), self.rect.y(), walls)) {
                self.rect.set_x(self.rect.x() + self.vx.signum());
            }
            self.vx = 0;
        }

        if self.collide(self.rect.x(), self.rect.y() + self.vy, walls) {
            while !self.collide(self.rect.x(), self.rect.y() + self.vy.signum(), walls) {
                self.rect.set_y(self.rect.y() + self.vy.signum());
            }
            self.vy = 0;
        }

        self.rect.set_x(self.rect.x() + self.vx);
        self.rect.set_y(self.rect.y() + self.vy);
        self.vx = 0;
        self.vy = 0;
    }

    pub fn mv(&mut self, dx: i32, dy: i32) {
        self.vx += dx;
        self.vy += dy;
    }

    fn collide(&self, x: i32, y: i32, walls: &[Wall]) -> bool {
        // Moved player collider
        let mut rect = self.rect.clone();
        rect.set_x(x);
        rect.set_y(y);
        //rect.resize(16, 16);

        // Find collision object
        for wall in walls {
            if rect.has_intersection(wall.collider) {
                return wall.solid();
            }
        }

        // No collisions
        false
    }
}
impl TypeInfo for Player<'_> {
    fn type_of(&self) -> &'static str {
        "Player"
    }
}

// Obstacles
pub struct Wall<'a> {
    sprite: Surface<'a>,
    rect: Rect,
    collider: Rect,
    solid: bool,
}

impl<'a> Wall<'a> {
    pub fn new(x: i32, y: i32, solid: bool, path: &std::path::Path) -> Self {
        let image: Surface = sdl2::image::LoadSurface::from_file(path).unwrap();
        let mut rect = image.rect();
        rect.set_x(x);
        rect.set_y(y);
        let mut collider = rect.clone();
        //collider.resize(rect.width() * 2, rect.height() * 2);
        Self {
            sprite: image,
            rect: rect,
            collider: collider,
            solid: solid,
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let texture_creator = canvas.texture_creator();
        let texture = self.sprite.as_texture(&texture_creator).unwrap();
        
        canvas.copy(&texture, None, self.rect.clone()).expect("Could not render wall");
    }

    pub fn solid(&self) -> bool {
        self.solid
    }

    pub fn set_solid(&mut self, solid: bool) {
        self.solid = solid;
    }
}
impl TypeInfo for Wall<'_> {
    fn type_of(&self) -> &'static str {
        "Wall"
    }
}
