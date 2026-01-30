use std::fs;

use the_raytracer_challenge_rust::primitives::{canvas::{self, Canvas}, color::Color, environment::{self, Environment}, point::Point, projectile::{self, Projectile}, vector::Vector};


pub struct ProjectileProgram {
    pub canvas: Canvas,
    projectile: Projectile,
    environment: Environment
}

impl ProjectileProgram {
    pub fn new(w: usize, h: usize, initial_projectile: Projectile, environment: Environment) -> Self {
        return ProjectileProgram { canvas:  Canvas::new(w, h), projectile: initial_projectile, environment: environment };
    }

    pub fn tick(&mut self) {
        self.canvas.write_pixel(
            self.projectile.position.X().round() as usize, 
            self.canvas.height - self.projectile.position.Y().round() as usize, 
            Color::new(1.0, 0.0, 0.0)
        );
        let v = self.projectile.position.clone() + self.projectile.velocity.clone();
        self.projectile.position =  Point::new(v.X(), v.Y(), v.Z());
        self.projectile.velocity = self.projectile.velocity.clone() + self.environment.gravity.clone() + self.environment.wind.clone();
    }

    pub fn should_continue(&self) -> bool{
        self.projectile.position.Y() > 0.0
    }
}

fn main() {
    let w:usize = 900;
    let h:usize = 550;
    let position = Point::new(0.0, 1.0, 0.0);
    let velocity = Vector::new(1.0, 1.8, 0.0).normalize() * 11.25;
    let gravity = Vector::new(0.0, -0.1, 0.0);
    let wind = Vector::new(-0.01, 0.0, 0.0);
    let e = Environment::new(gravity, wind);
    let projectile = Projectile { position, velocity  };
    let mut program = ProjectileProgram::new(w, h, projectile, e);
    while program.should_continue() {
        program.tick();
    }

    let file_path = "output.ppm";
    let content =  program.canvas.to_ppm();
    fs::write(file_path, content);
  

}
