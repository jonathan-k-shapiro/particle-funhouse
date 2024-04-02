use nannou::noise::{NoiseFn, Seedable};
use nannou::prelude::*;
use super::particle::Particle;
use log::*;

#[derive(Debug)]
pub struct Emitter {
    pub particles: Vec<Particle>,
    initializer: fn(Bounds) -> Particle,
    noise_field: Option<nannou::noise::Perlin>,
    bounds: Bounds,
    paused: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct Bounds {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

pub fn default_initializer(_: Bounds) -> Particle {
    let pos = pt2(0.0, 0.0);
    let vel = vec2(random_f32() * 2.0 - 1.0, random_f32() - 1.0);
    let hue = random_f32() * 360.0;
    let radius = 4.0;
    let life_span = 255.0;
    Particle::new(pos, vel, hue, radius, life_span)
}

impl Emitter {
    pub fn new(bounds: Bounds) -> Self {
        Emitter {
            particles: Vec::new(),
            initializer: default_initializer,
            noise_field: None,
            bounds,
            paused: false,
            
        }
    }

    pub fn with_initializer(bounds: Bounds, initializer: fn(Bounds) -> Particle) -> Self {
        Emitter {
            particles: Vec::new(),
            initializer,
            noise_field: None,
            bounds,
            paused: false,
        }
    }

    pub fn with_noise_field(bounds: Bounds, initializer: fn(Bounds) -> Particle) -> Self {
        let noise = nannou::noise::Perlin::new();
        Emitter {
            particles: Vec::new(),
            initializer,
            noise_field: Some(noise),
            bounds,
            paused: false,
        }
    }

    pub fn emit(&mut self) {
        if self.paused {
            debug!("Emitter is paused");
            return;
        }
        for _ in 0..10 {
            self.particles.push((self.initializer)(self.bounds));
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        for p in self.particles.iter_mut() {
            p.apply_force(force);
        }
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn update(&mut self) {
        for i in (0..self.particles.len()).rev() {
            match &self.noise_field {
                Some(noise) => {
                    let angle = TAU * noise.get([
                        self.particles[i].position.x as f64/500.0, //opt noise_scale
                        self.particles[i].position.y as f64/500.0,
                        0.0,
                    ])  as f32;
                    let dir = vec2(angle.cos(), angle.sin());
                    trace!("{:?}, {:?}", angle, dir);
                    self.particles[i].update(Some(dir / 50.0));
                },
                None => {
                    self.particles[i].update(None);
                }
            }   
            
            if self.particles[i].is_dead() {
                self.particles.remove(i);
            }
        }
    }

    pub fn display(&self, draw: &Draw) {
        for p in self.particles.iter() {
            p.display(draw);
        }
    }
}   