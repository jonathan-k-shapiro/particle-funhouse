use serde::*;
use std::fs;
use std::collections::HashMap;
use toml;
use nannou::prelude::*;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub capture_prefix: Option<String>,
    pub color_pickers: Option<HashMap<String, ColorPickerConfig>>,
    pub emitters: Option<HashMap<String, EmitterConfig>>,
}


#[derive(Debug, Deserialize, Clone)]
pub struct ColorPickerConfig {
    pub hue: Option<f32>,
    pub saturation: Option<f32>,
    pub lightness: Option<f32>,
    pub alpha: Option<f32>,
    pub range_hue: Option<Vec2>,
    pub range_saturation: Option<Vec2>,
    pub range_lightness: Option<Vec2>,
    pub range_alpha: Option<Vec2>,
    pub num_colors: Option<usize>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmitterConfig {
    pub randomize_position: Option<bool>,
    pub position: Option<Vec2>,
    pub velocity: Option<Vec2>,
    pub life_span: Option<f32>,
    pub color_picker: Option<String>,
}

pub fn read_config(filename: &str) -> Config {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let config: Config = toml::from_str(&contents).unwrap();
    config
}

#[cfg(test)]
mod tests {
    use std::vec;
    use tempfile::NamedTempFile;

    use super::*;
    #[test]
    fn test_basic() {
        const TEXT: &str = r#"
        hue = 120
        saturation = 0.3
        lightness = 0.7
        alpha = 1.0
        num_colors = 5
        "#;

        let config: ColorPickerConfig = toml::from_str(TEXT).unwrap();
        println!("{:#?}", config);
        // assert_eq!(config.hue.unwrap(), 120.0);
    }

    #[test]
    fn test_read_nested() {
        const TEXT: &str = r#"
        capture_prefix = "particle_"

        [color_pickers]
          [color_pickers.mono_green]
            name = "mono_green"
            hue = 120
            range_saturation = [0.3, 0.7]
            some_bullshit = 0.7
        "#;

        let config: Config = toml::from_str(TEXT).unwrap();
        println!("{:#?}", config);
        assert_eq!(config.color_pickers.unwrap()["mono_green"].hue.unwrap(), 120.0);
    }

    #[test]
    fn test_read_file() {
        const TEXT: &str = r#"
        capture_prefix = "particle_"

        [color_pickers]
          [color_pickers.mono_green]
            hue = 120
            range_saturation = [0.3, 0.7]
            range_lightness = [0.3, 0.7]
        
        [emitters]
          [emitters.emitter_1]
            position = [0, 0]
            velocity = [0, 0]
            life_span = 512
            color_picker = "mono_green"
        "#;

        fs::write("/tmp/conf", TEXT).expect("Unable to write file");

        let config: Config = read_config("/tmp/conf");
        println!("{:#?}", config);
        assert!(config.color_pickers.unwrap().contains_key("mono_green"));
        // assert_eq!(config.color_pickers.unwrap()["mono_green"].range_saturation.unwrap(), vec2(0.3, 0.7));
    }
}
