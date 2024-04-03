use super::color_picker::ColorPicker;
use super::config::EmitterConfig;
use super::particle::Particle;
use super::CONFIG;
use log::*;
use nannou::noise::{NoiseFn, Seedable};
use nannou::prelude::*;

#[derive(Debug)]
pub struct Emitter {
    bounds: Bounds,
    color_picker: ColorPicker,
    flight_size: usize,
    pub life_span: f32,
    noise_field: Option<nannou::noise::Perlin>,
    noise_scale: f64,
    noise_strength: f32,
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
            particles: Vec::new(),
            noise_field: None,
            noise_scale: 0.0,
            noise_strength: 0.0,
            position: pt2(0.0, 0.0),
            randomize_position: false,
            randomize_velocity: true,
            flight_size: 10,
            life_span: 512.0,
            radius: 10.0,
            stroke_weight: 2.0,
            bounds,
            paused: false,
            color_picker,
            visualize_noise_field: true,
        }
    }

    pub fn from_config(config: EmitterConfig, bounds: Bounds) -> Self {
        let color_picker_name: String = config.color_picker.unwrap_or("".to_string());
        debug!(
            "cp key {:?} exists: {:?}",
            &color_picker_name,
            CONFIG
                .color_pickers
                .as_ref()
                .unwrap()
                .contains_key(&color_picker_name)
        );
        let color_picker = if CONFIG
            .color_pickers
            .as_ref()
            .unwrap()
            .contains_key(&color_picker_name)
        {
            ColorPicker::from_config(
                CONFIG.color_pickers.as_ref().unwrap()[&color_picker_name].clone(),
            )
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
        let randomize_position = config.randomize_position.unwrap_or(false);
        let randomize_velocity = config.randomize_velocity.unwrap_or(true);
        let life_span = config.life_span.unwrap_or(512.0);
        let noise_field_on = config.noise_field.unwrap_or(false);
        let noise_scale = config.noise_scale.unwrap_or(0.0);
        let noise_strength = config.noise_strength.unwrap_or(0.0);
        let position = config.position.unwrap_or(pt2(0.0, 0.0));
        let flight_size = config.flight_size.unwrap_or(10);
        let radius = config.radius.unwrap_or(10.0);
        let stroke_weight = config.stroke_weight.unwrap_or(2.0);
        let visualize_noise_field = config.visualize_noise_field.unwrap_or(false);
        debug!("visualize_noise_field: {:?}", visualize_noise_field);

        let noise_field = if noise_field_on {
            let seed = CONFIG.seed.unwrap_or(0);
            Some(nannou::noise::Perlin::new().set_seed(seed))
        } else {
            None
        };
        Emitter {
            particles: Vec::new(),
            noise_field,
            noise_scale,
            noise_strength,
            position,
            radius,
            stroke_weight,
            randomize_position,
            randomize_velocity,
            flight_size,
            life_span,
            bounds,
            paused: false,
            color_picker,
            visualize_noise_field
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
            vec2(0.0, 0.0)
        };

        let color = self.color_picker.get_next_color();
        trace!("color picked: {:?}", color);
        Particle::new(
            pos,
            vel,
            color,
            self.radius,
            self.stroke_weight,
            self.life_span,
        )
    }

    pub fn emit(&mut self) {
        if self.paused {
            debug!("Emitter is paused");
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
        for i in (0..self.particles.len()).rev() {
            match &self.noise_field {
                Some(noise) => {
                    let angle = TAU
                        * noise.get([
                            self.particles[i].position.x as f64 * self.noise_scale, 
                            self.particles[i].position.y as f64 * self.noise_scale,
                            // _t as f64 * self.noise_scale,
                            0.0 as f64
                        ]) as f32;
                    let dir = vec2(angle.cos(), angle.sin());
                    trace!("{:?}, {:?}", angle, dir);
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
        let step : f32 = 10.0;
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
