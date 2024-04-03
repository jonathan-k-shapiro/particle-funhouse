use super::config::ColorPickerConfig;
use nannou::prelude::*;

#[derive(Debug, Clone)]
pub struct ColorPicker {
    pub hue: f32,   //0..360
    pub sat: f32,   //0..1
    pub light: f32, //0..1
    pub alpha: f32, //0..1
    pub range_hue: Option<Vec2>,
    pub range_sat: Option<Vec2>,
    pub range_light: Option<Vec2>,
    pub range_alpha: Option<Vec2>,
    pub num_colors: usize,
    colors: Option<Vec<Hsla>>,
    current_color: usize,
}

impl ColorPicker {
    pub fn new(
        num_colors: usize,
        hue: f32,
        sat: f32,
        light: f32,
        alpha: f32,
        range_hue: Option<Vec2>,
        range_sat: Option<Vec2>,
        range_light: Option<Vec2>,
        range_alpha: Option<Vec2>,
    ) -> Self {
        ColorPicker {
            hue,
            sat,
            light,
            alpha,
            range_hue,
            range_sat,
            range_light,
            range_alpha,
            num_colors,
            colors: None,
            current_color: 0,
        }
    }

    pub fn from_config(config: ColorPickerConfig) -> Self {
        let hue = config.hue.unwrap_or(0.0);
        let sat = config.saturation.unwrap_or(0.5);
        let light = config.lightness.unwrap_or(0.5);
        let alpha = config.alpha.unwrap_or(1.0);
        let range_hue = config.range_hue;
        let range_sat = config.range_saturation;
        let range_light = config.range_lightness;
        let range_alpha = config.range_alpha;
        let num_colors = config.num_colors.unwrap_or(1);
        ColorPicker {
            hue,
            sat,
            light,
            alpha,
            range_hue,
            range_sat,
            range_light,
            range_alpha,
            num_colors,
            colors: None,
            current_color: 0,
        }
    }

    pub fn get_next_color(&mut self) -> Hsla {
        self.initialize();
        self.current_color += 1;
        if self.current_color >= self.num_colors {
            self.current_color = 0;
        }
        self.colors.as_ref().unwrap()[self.current_color]
    }

    fn initialize(&mut self) {
        match self.colors {
            Some(_) => {}
            None => {
                self.colors = Some(self.get_colors(self.num_colors));
            }
        }
    }

    fn get_colors(&self, n: usize) -> Vec<Hsla> {
        let mut colors: Vec<Hsla> = Vec::new();
        let hues = match self.range_hue {
            Some(range_hue) => gen_values(n, range_hue),
            None => vec![self.hue; n],
        };
        let sats = match self.range_sat {
            Some(range_sat) => gen_values(n, range_sat),
            None => vec![self.sat; n],
        };
        let lights = match self.range_light {
            Some(range_light) => gen_values(n, range_light),
            None => vec![self.light; n],
        };
        let alphas = match self.range_alpha {
            Some(range_alpha) => gen_values(n, range_alpha),
            None => vec![self.alpha; n],
        };
        for i in 0..n {
            colors.push(Hsla::new(hues[i], sats[i], lights[i], alphas[i]));
        }
        colors
    }
}

fn gen_values(n: usize, range: Vec2) -> Vec<f32> {
    let mut values = Vec::new();
    let golden_ratio_conjugate = 0.618033988749895;
    let mut h = random_f32();
    for _ in 0..n {
        h += golden_ratio_conjugate;
        h %= 1.0;
        let value = h * (range[1] - range[0]) + range[0];
        values.push(value);
    }
    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut cp = ColorPicker::new(10, 0.5, 0.5, 0.5, 1.0, None, None, None, None);
        for _ in 0..10 {
            let color = cp.get_next_color();
            println!("{:?}", color);
        }
        assert_eq!(1, 1);
    }

    #[test]
    fn test_randomized() {
        let mut cp = ColorPicker::new(
            10,
            0.5,
            0.5,
            0.5,
            1.0,
            Some(vec2(0.0, 1.0)),
            None,
            None,
            None,
        );
        for _ in 0..10 {
            let mut color = cp.get_next_color();
            color.alpha = 0.5;
            println!("{:?}, {:?}", color.hue, color.alpha);
        }
        assert_eq!(1, 1);
    }
}
