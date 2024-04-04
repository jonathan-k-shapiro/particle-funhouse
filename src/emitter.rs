use super::color_picker::ColorPicker;
use super::config::{ColorPickerConfig, EmitterConfig, MoverConfig};
use super::mover::Mover;
use super::particle::Particle;

use log::*;
use nannou::noise::{NoiseFn, Seedable};
use nannou::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Emitter {
    name: String,
    bounds: Bounds,
    color_picker: ColorPicker,
    flight_size: usize,
    initial_velocity: Vec2,
    pub life_span: f32,
    mover: Option<Mover>,
    noise_field: Option<nannou::noise::Perlin>,
    noise_scale: f64,
    noise_strength: f32,
    origin: Point2,
    pub particles: Vec<Particle>,
    paused: bool,
    position: Point2,
    radius: f32,
    pub randomize_position: bool,
    pub randomize_velocity: bool,
    stroke_weight: f32,
    visualize_noise_field: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct Bounds {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

impl Emitter {
    pub fn new(bounds: Bounds) -> Self {
        let color_picker = ColorPicker::new(
            10,
            117.,
            0.5,
            0.5,
            0.5,
            None,
            Some(vec2(0.3, 0.7)),
            Some(vec2(0.3, 0.7)),
            None,
        );
        debug!("color picker: {:#?}", color_picker);
        Emitter {
            name: "unnamed_emitter".to_string(),
            particles: Vec::new(),
            mover: None,
            noise_field: None,
            noise_scale: 0.0,
            noise_strength: 0.0,
            origin: pt2(0.0, 0.0),
            position: pt2(0.0, 0.0),
            randomize_position: false,
            randomize_velocity: true,
            flight_size: 10,
            initial_velocity: vec2(0.0, 0.0),
            life_span: 512.0,
            radius: 10.0,
            stroke_weight: 2.0,
            bounds,
            paused: false,
            color_picker,
            visualize_noise_field: true,
        }
    }

    fn color_picker_from_config(
        color_picker_name: &str,
        color_pickers: &HashMap<String, ColorPickerConfig>,
    ) -> ColorPicker {
        // let color_picker_name: String = config.color_picker.unwrap_or("".to_string());
        let color_picker = if color_pickers.contains_key(color_picker_name) {
            ColorPicker::from_config(color_picker_name.to_string(), color_pickers[color_picker_name].clone())
        } else {
            ColorPicker::new(
                1,
                1.,
                0.5,
                0.5,
                0.5,
                None,
                Some(vec2(0.3, 0.7)),
                Some(vec2(0.3, 0.7)),
                None,
            )
        };
        color_picker
    }

    fn mover_from_config(mover_name: &str, movers: &HashMap<String, MoverConfig>) -> Option<Mover> {
        let mover = if movers.contains_key(mover_name) {
            Some(Mover::from_config(mover_name.to_string(), movers[mover_name].clone()))
        } else {
            None
        };
        mover
    }

    pub fn from_config(
        name: String,
        config: EmitterConfig,
        color_pickers_config: &HashMap<String, ColorPickerConfig>,
        movers_config: &HashMap<String, MoverConfig>,
        bounds: Bounds,
        seed: u32,
    ) -> Self {
        let color_picker_name = config.color_picker.unwrap_or("".to_string());
        let color_picker = Self::color_picker_from_config(&color_picker_name, color_pickers_config);
        let mover_name = config.mover.unwrap_or("".to_string());
        let mover = Self::mover_from_config(&mover_name, movers_config);
        let randomize_position = config.randomize_position.unwrap_or(false);
        let randomize_velocity = config.randomize_velocity.unwrap_or(true);
        let initial_velocity = config.initial_velocity.unwrap_or(vec2(0.0, 0.0));
        let life_span = config.life_span.unwrap_or(512.0);
        let noise_field_on = config.noise_field.unwrap_or(false);
        let noise_scale = config.noise_scale.unwrap_or(0.0);
        let noise_strength = config.noise_strength.unwrap_or(0.0);
        let origin = config.origin.unwrap_or(pt2(0.0, 0.0));
        let flight_size = config.flight_size.unwrap_or(10);
        let radius = config.radius.unwrap_or(10.0);
        let stroke_weight = config.stroke_weight.unwrap_or(2.0);
        let visualize_noise_field = config.visualize_noise_field.unwrap_or(false);
        debug!("[{:?}] visualize_noise_field: {:?}", name, visualize_noise_field);

        let noise_field = if noise_field_on {
            Some(nannou::noise::Perlin::new().set_seed(seed))
        } else {
            None
        };

        debug!("[{:?}] mover: {:?}\ncolor_picker: {:?}", name, mover, color_picker);
        Emitter {
            name,
            particles: Vec::new(),
            mover,
            noise_field,
            noise_scale,
            noise_strength,
            origin,
            position: origin,
            radius,
            stroke_weight,
            randomize_position,
            randomize_velocity,
            flight_size,
            initial_velocity,
            life_span,
            bounds,
            paused: false,
            color_picker,
            visualize_noise_field,
        }
    }

    pub fn initializer(&mut self, _: Bounds) -> Particle {
        let mut pos = self.position;
        if self.randomize_position {
            let w = self.bounds.right - self.bounds.left;
            let h = self.bounds.top - self.bounds.bottom;
            pos = pt2(
                ((random_f32() * 2. - 1.) * w / 2.).floor(),
                ((random_f32() * 2. - 1.) * h / 2.).floor(),
            );
        }
        let vel = if self.randomize_velocity {
            // todo: make different types of randomizers
            vec2(random_f32() * 2.0 - 1.0, random_f32() * 2.0 - 1.0)
        } else {
            self.initial_velocity
        };

        let color = self.color_picker.get_next_color();
        trace!("[{:?}] color picked: {:?}", self.name, color);
        let mut particle = Particle::new(
            pos,
            vel,
            color,
            self.radius,
            self.stroke_weight,
            self.life_span,
        );
        // // Apply a one-time 'gravitational' force
        // particle.apply_force(vec2(0.0, -0.02));
        particle
    }

    pub fn emit(&mut self) {
        if self.paused {
            trace!("[{:?}] Emitter is paused", self.name);
            return;
        }
        for _ in 0..self.flight_size {
            let p = self.initializer(self.bounds);
            self.particles.push(p);
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

    pub fn update(&mut self, _t: f32) {
        // Move the emitter
        match self.mover {
            Some(ref m) => {
                self.position = m.get_postion(_t);
                trace!("[{:?}] position: {:?}", self.name, self.position)
            }
            _ => {}
        }

        for i in (0..self.particles.len()).rev() {
            match &self.noise_field {
                Some(noise) => {
                    let angle = TAU
                        * noise.get([
                            self.particles[i].position.x as f64 * self.noise_scale,
                            self.particles[i].position.y as f64 * self.noise_scale,
                            // _t as f64 * self.noise_scale,
                            0.0 as f64,
                        ]) as f32;
                    let dir = vec2(angle.cos(), angle.sin());
                    trace!("[{:?}] angle:{:?}, dir:{:?}", self.name, angle, dir);
                    self.particles[i].update(Some(dir * self.noise_strength));
                }
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
        if self.visualize_noise_field {
            self.draw_flow_field(draw);
        }
    }

    pub fn draw_flow_field(&self, draw: &Draw) {
        let step: f32 = 10.0;
        for x in (self.bounds.left as i32..self.bounds.right as i32).step_by(step as usize) {
            for y in (self.bounds.bottom as i32..self.bounds.top as i32).step_by(step as usize) {
                let angle = TAU
                    * self.noise_field.as_ref().unwrap().get([
                        x as f64 * self.noise_scale,
                        y as f64 * self.noise_scale,
                        0.0 as f64,
                    ]) as f32;
                let dir = vec2(angle.cos(), angle.sin());
                draw.arrow()
                    .start(pt2(x as f32, y as f32))
                    .end(pt2(x as f32 + dir.x * 10.0, y as f32 + dir.y * 10.0))
                    .weight(1.0)
                    .color(BLUE);
            }
        }
    }
}
