use nannou::prelude::*;
use nannou::Draw;

#[derive(Debug)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub radius: f32,
    pub stroke_weight: f32,
    pub life_span: f32,
    pub init_life_span: f32,
    pub color: Hsla,
}

impl Particle {
    pub fn new(
        pos: Point2,
        velocity: Vec2,
        color: Hsla,
        radius: f32,
        stroke_weight: f32,
        life_span: f32,
    ) -> Self {
        let velocity = velocity;
        let position = pos;
        let acceleration = vec2(0.0, 0.0);
        let radius = radius;
        let life_span = life_span;
        let init_life_span = life_span;
        let color = color;
        Particle {
            acceleration,
            velocity,
            position,
            radius,
            stroke_weight,
            life_span,
            init_life_span,
            color,
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    // Method to update position
    pub fn update(&mut self, direction: Option<Vec2>) {
        match direction {
            Some(dir) => {
                self.velocity += dir;
            }
            None => {}
        }
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.life_span -= 2.0;
    }

    // Method to display
    pub fn display(&self, draw: &Draw) {
        let r = self.radius * self.life_span / self.init_life_span;
        let mut color = self.color;
        color.alpha = self.life_span / self.init_life_span;
        draw.ellipse()
            .xy(self.position)
            .w_h(r, r)
            .color(color)
            .stroke(rgba(0.0, 0.0, 0.0, self.life_span / self.init_life_span))
            .stroke_weight(self.stroke_weight);
        // self.draw_velocity(draw);
    }

    pub fn draw_velocity(&self, draw: &Draw) {
        draw.arrow()
            .start(self.position)
            .end(self.position + self.velocity)
            .color(RED)
            .stroke_weight(1.0);
    }

    pub fn is_dead(&self) -> bool {
        if self.life_span < 0.0 {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let p = Particle::new(
            pt2(0.0, 0.0),
            vec2(1., 1.),
            hsla(0.5, 0.5, 0.5, 1.),
            4.0,
            2.0,
            255.0,
        );
        assert_eq!(p.position, pt2(0.0, 0.0));
        assert_eq!(p.velocity, vec2(1., 1.));
        assert_eq!(p.acceleration, vec2(0.0, 0.0));
        assert_eq!(p.radius, 4.0);
        assert_eq!(p.life_span, 255.0);
        assert_eq!(p.init_life_span, 255.0);
        // assert_eq!(p.hue, 0.0);
    }

    #[test]
    fn test_update() {
        let mut p = Particle::new(
            pt2(0.0, 0.0),
            vec2(1., 1.),
            hsla(0.5, 0.5, 0.5, 1.),
            4.0,
            2.0,
            255.0,
        );
        p.update(None);
        assert_eq!(p.position, pt2(1.0, 1.0));
    }

    #[test]
    fn test_apply_force() {
        let mut p = Particle::new(
            pt2(0.0, 0.0),
            vec2(1., 1.),
            hsla(0.5, 0.5, 0.5, 1.),
            4.0,
            2.0,
            255.0,
        );
        p.apply_force(vec2(1., 1.));
        p.update(None);
        assert_eq!(p.acceleration, vec2(1., 1.));
        assert_eq!(p.velocity, vec2(2., 2.));
    }

    #[test]
    fn test_update_with_direction() {
        let mut p = Particle::new(
            pt2(0., 0.),
            vec2(1., 1.),
            hsla(0.5, 0.5, 0.5, 1.),
            4.0,
            2.0,
            255.0,
        );
        p.update(Some(vec2(1., 1.)));
        assert_eq!(p.acceleration, vec2(0., 0.));
        assert_eq!(p.velocity, vec2(2., 2.));
    }
}
