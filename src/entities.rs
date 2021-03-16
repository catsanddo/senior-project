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
    ax: f32,
    ay: f32,
    vx: i32,
    pub vy: i32,
    flip: bool,
    pub jump: bool,
    pub frame: f32,
    pub attack: bool,
}

impl<'a> Player<'a> {
    pub fn new(x: i32, y: i32, path: &std::path::Path) -> Self {
        let image: Surface = sdl2::image::LoadSurface::from_file(path).unwrap();
        let rect = Rect::new(x, y, 8, 12);
        Self {
            sprite: image,
            rect: rect,
            ax: x as f32,
            ay: y as f32,
            vx: 0,
            vy: 0,
            flip: false,
            jump: true,
            frame: 0.0,
            attack: false,
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let texture_creator = canvas.texture_creator();
        let texture = self.sprite.as_texture(&texture_creator).unwrap();
        let src_rect = Rect::new(8 * self.frame as i32, 0, 8, 12);

        canvas.copy_ex(&texture, src_rect, self.rect.clone(), 0.0, None, self.flip, false).expect("Could not render player");
        
        // Draw weapon when attacking
        if self.attack {
            let src_rect = Rect::new(8 * (self.frame as i32 + 3), 0, 8, 12);
            let mut dest_rect = self.rect.clone();
            if self.flip {
                dest_rect.set_x(dest_rect.x() - 8);
            } else {
                dest_rect.set_x(dest_rect.x() + 8);
            }
            canvas.copy_ex(&texture, src_rect, dest_rect, 0.0, None, self.flip, false).expect("Could not render player");
        }
    }

    pub fn update(&mut self, delta_time: f32, walls: &mut Vec<Wall>) {
        // Animation
        if self.attack {
                self.frame += 11.0 * delta_time;
                if self.frame >= 7.0 {
                    self.attack = false;
                    self.frame = 0.0;
                }
        } else {
            if self.vx != 0 {
                // Walk cycle at 11 FPS
                self.frame += 11.0 * delta_time;
                if self.frame >= 4.0 {
                    self.frame = 0.0;
                }
                if self.vx < 0 {
                    self.flip = true;
                } else if self.vx > 0 {
                    self.flip = false;
                }
            } else {
                self.frame = 0.0;
            }
        }

        // Gravity
        self.vy += 15;

        let dx = self.vx as f32 * delta_time;
        let dy = self.vy as f32 * delta_time;
        // Movement
        if self.collide((self.ax + dx) as i32, self.rect.y(), walls) {
            while !(self.collide(self.rect.x() + self.vx.signum(), self.rect.y(), walls)) {
                self.rect.set_x(self.rect.x() + self.vx.signum());
            }
            self.vx = 0;
        }

        if self.collide(self.rect.x(), (self.ay + dy) as i32, walls) {
            while !self.collide(self.rect.x(), self.rect.y() + self.vy.signum(), walls) {
                self.rect.set_y(self.rect.y() + self.vy.signum());
            }
            if self.vy > 0 { self.jump = true; }
            self.vy = 0;
        }

        self.ax += self.vx as f32 * delta_time;
        self.ay += self.vy as f32 * delta_time;
        self.rect.set_x(self.ax as i32);
        self.rect.set_y(self.ay as i32);
        self.vx = 0;

        // Attacking
        if self.attack {
            let mut attack_collider = self.rect.clone();
            if self.flip {
                attack_collider.set_x(attack_collider.x() - 8);
            } else {
                attack_collider.set_x(attack_collider.x() + 8);
            }
            
            for i in 0..walls.len() {
                if attack_collider.has_intersection(walls[i].collider) {
                    walls.remove(i);
                    break;
                }
            }
        }
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
        let collider = rect.clone();
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
