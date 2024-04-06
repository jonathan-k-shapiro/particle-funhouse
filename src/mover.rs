use super::config::MoverConfig;
use nannou::prelude::*;

fn dot(a: Vec2, b: Vec2) -> Vec2 {
    vec2(a.x * b.x, a.y * b.y)
} 

#[derive(Debug, Clone, Copy)]
pub struct MoverParams {
    pub inner: Vec2,
    pub outer: Vec2,
    pub scale: Vec2,
    pub translation: Vec2,
    pub rotation_angle: f32,
    pub rotation_speed: f32,
}
pub type MoverFn = fn(f32, MoverParams) -> Point2;

#[derive(Debug, Clone)]
pub struct Mover {
    pub name: String,
    pub params: MoverParams,
    pub mover: MoverFn,
}

impl Mover {
    pub fn from_config(name: String, config: MoverConfig) -> Self {
        let mover = match config.mover_type.as_str() {
            "p_elipse" => p_elipse,
            _ => p_elipse,
        };
        let params = MoverParams {
            inner: config.inner,
            outer: config.outer,
            scale: config.scale,
            translation: config.translation.unwrap_or(vec2(0., 0.)),
            rotation_angle: config.rotation_angle.unwrap_or(0.),
            rotation_speed: config.rotation_speed.unwrap_or(0.),
        };
        Mover {
            name,
            params,
            mover,
        }
    }

    pub fn get_postion(&self, t: f32) -> Point2 {
        let position = (self.mover)(t, self.params)
            .rotate(self.params.rotation_angle + t * self.params.rotation_speed);
        position + self.params.translation
    }
}

pub fn p_elipse(t:f32, params: MoverParams) -> Point2 {
    let inner = params.inner;
    let outer = params.outer;
    let scale = params.scale;
    let cosine = (inner.x * t).cos() * outer.x;
    let sine = (inner.y * t).sin() * outer.y;
    let bounds = Rect::from_w_h(scale.x, scale.y);
    let x = map_range(cosine, -1.0, 1.0, bounds.left(), bounds.right());
    let y = map_range(sine, -1.0, 1.0, bounds.bottom(), bounds.top());
    pt2(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p_elipse() {
        let p = p_elipse(0.0, MoverParams {
            inner: vec2(1., 1.),
            outer: vec2(1., 1.),
            scale: vec2(50., 100.),
            translation: vec2(0., 0.),
            rotation_angle: 0.0,
            rotation_speed: 0.0, 
        });
        assert_eq!(p, pt2(25.0, 0.0));
    }
}

// --- Experimental code below here. Not yet set up for emitters ---

pub fn epicycloid(t: f32, t_min: f32, t_max: f32, a: f32, b: f32, scale: Vec2) ->Vec2 {
    let t_range = t_max - t_min;
    let _t = ( t % t_range) - t_range / 2.;
    let x = (a + b) * _t.cos() - (b + 1.)  * ((a / b + 1.) * _t).cos();
    let y = (a + b) * _t.sin() - (b + 1.)  * ((a / b + 1.) * _t).sin();
    vec2(x, y) * scale
}

pub type Transform = fn(f32, f32, Vec2) -> Vec2;

pub fn p_trig(t:f32, t_min: f32, t_max: f32, inner: Vec2, outer: Vec2, transform: Transform, scale: Vec2) -> Vec2 {
    // let t_range = t_max - t_min;
    // let _t = ( t % t_range) - t_range / 2.;
    let _t = t;
    let scaled_t = dot(vec2(_t, _t), inner);
    let cosine = scaled_t.x.cos();
    let sine = scaled_t.y.sin();
    let x_y = transform(cosine, sine, outer);
    let scaled_x_y = dot(x_y, scale);
    scaled_x_y
}

pub fn identity_transform(c: f32, s: f32, coefs: Vec2) -> Vec2 {
    let scaled_c_s = dot(vec2(c, s), coefs);
    scaled_c_s
}

pub fn test_transform_2(c: f32, s: f32, coefs: Vec2) -> Vec2 {
    let scaled_c_s = dot(vec2(c, s), coefs);
    let x = pow(scaled_c_s.x, 3) / scaled_c_s.y;
    let y = pow(scaled_c_s.y, 2);
    vec2(x, y)
}

pub fn test_transform_3(c: f32, s: f32, coefs: Vec2) -> Vec2 {
    let scaled_c_s = dot(vec2(c, s), coefs);
    let x = scaled_c_s.y / (1. + pow(scaled_c_s.x, 2));
    let y = scaled_c_s.x * pow(scaled_c_s.y, 4);
    vec2(x, y)
}



