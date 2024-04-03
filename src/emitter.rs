use super::color_picker::ColorPicker;
use super::config::EmitterConfig;
use super::particle::Particle;
use super::CONFIG;
use log::*;
use nannou::noise::NoiseFn;
use nannou::prelude::*;

#[derive(Debug)]
pub struct Emitter {
    pub particles: Vec<Particle>,
    noise_field: Option<nannou::noise::Perlin>,
    pub randomize_position: bool,
    flight_size: usize,
    pub life_span: f32,
    radius: f32,
    stroke_weight: f32,
    bounds: Bounds,
    paused: bool,
    color_picker: ColorPicker,
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
        // let color_picker = ColorPicker::new(10, 117., 0.5, 0.5, 0.5, None, Some(vec2(0.3, 0.7)), Some(vec2(0.3, 0.7)), None);
        let color_picker =
            ColorPicker::from_config(CONFIG.color_pickers.as_ref().unwrap()["mono_green"].clone());
        debug!("color picker: {:#?}", color_picker);
        Emitter {
            particles: Vec::new(),
            noise_field: None,
            randomize_position: false,
            flight_size: 10,
            life_span: 512.0,
            radius: 10.0,
            stroke_weight: 2.0,
            bounds,
            paused: false,
            color_picker,
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
        let life_span = config.life_span.unwrap_or(512.0);
        let flight_size = config.flight_size.unwrap_or(10);
        let radius = config.radius.unwrap_or(10.0);
        let stroke_weight = config.stroke_weight.unwrap_or(2.0);
        debug!("randomize_position: {:?}", randomize_position);
        Emitter {
            particles: Vec::new(),
            noise_field: None,
            radius,
            stroke_weight,
            randomize_position,
            flight_size,
            life_span,
            bounds,
            paused: false,
            color_picker,
        }
    }

    pub fn initializer(&mut self, _: Bounds) -> Particle {
        let mut pos = pt2(0.0, 0.0);
        if self.randomize_position {
            let w = self.bounds.right - self.bounds.left;
            let h = self.bounds.top - self.bounds.bottom;
            pos = pt2(
                ((random_f32() * 2. - 1.) * w / 2.).floor(),
                ((random_f32() * 2. - 1.) * h / 2.).floor(),
            );
        }
        let vel = vec2(random_f32() * 2.0 - 1.0, random_f32() - 1.0);
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

    // pub fn with_initializer(bounds: Bounds, initializer: fn(Bounds) -> Particle) -> Self {
    //     Emitter {
    //         particles: Vec::new(),
    //         initializer,
    //         noise_field: None,
    //         bounds,
    //         paused: false,
    //     }
    // }

    // pub fn with_noise_field(bounds: Bounds, initializer: fn(Bounds) -> Particle) -> Self {
    //     let noise = nannou::noise::Perlin::new();
    //     Emitter {
    //         particles: Vec::new(),
    //         initializer,
    //         noise_field: Some(noise),
    //         bounds,
    //         paused: false,
    //     }
    // }

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

    pub fn update(&mut self) {
        for i in (0..self.particles.len()).rev() {
            match &self.noise_field {
                Some(noise) => {
                    let angle = TAU
                        * noise.get([
                            self.particles[i].position.x as f64 / 500.0, //opt noise_scale
                            self.particles[i].position.y as f64 / 500.0,
                            0.0,
                        ]) as f32;
                    let dir = vec2(angle.cos(), angle.sin());
                    trace!("{:?}, {:?}", angle, dir);
                    self.particles[i].update(Some(dir / 50.0)); // opt noise_strength
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
    }
}
