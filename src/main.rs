extern crate nannou;

use lazy_static::lazy_static;
use log::*;
use nannou::prelude::*;
use structopt::StructOpt;
use particle_lib::*;

lazy_static! {
    #[derive(Debug)]
    pub static ref OPT: Opt = Opt::from_args();
    pub static ref CONFIG: config::Config = config::read_config(&OPT.config_file);
}

#[derive(StructOpt, Debug)]
#[structopt(name = "basic_particle", about = "Particle system with Perlin noise.")]
pub struct Opt {
    /// Configuration file
    #[structopt(short, long, default_value = "")]
    config_file: String,   
}

struct Model {
    emitters: Vec<emitter::Emitter>,
}

fn model(_app: &App) -> Model {
    let default_window_size : f32 = 600.;
    let window_height = CONFIG.window_height.as_ref().unwrap_or(&default_window_size);
    let window_width =  CONFIG.window_width.as_ref().unwrap_or(&default_window_size); 
    _app.new_window()
        .size(window_width.clone() as u32, window_height.clone() as u32)  
        .key_released(key_released)
        .view(view)
        .build()
        .unwrap();

    let r = _app.window_rect().right();
    let l = _app.window_rect().left();

    let t = _app.window_rect().top();
    let b = _app.window_rect().bottom();
    let bounds: emitter::Bounds = emitter::Bounds {
        top: t,
        bottom: b,
        left: l,
        right: r,
    };

    let selected_emitters = match CONFIG.selected_emitters {
        Some(ref emitters) => emitters.clone(),
        None => vec!["default".to_string()],
    };
    let color_pickers = match CONFIG.color_pickers {
        Some(ref color_pickers) => color_pickers.clone(),
        None => std::collections::HashMap::new(),
    };
    let movers = match CONFIG.movers {
        Some(ref movers) => movers.clone(),
        None => std::collections::HashMap::new(),
    };
    let seed = CONFIG.seed.unwrap_or(0);
    let mut emitters = Vec::new();
    for e in selected_emitters.iter() {
        info!("emitter: {:?}", e);
        let emitter = emitter::Emitter::from_config(
            e.to_string(),
            CONFIG.emitters.as_ref().unwrap()[e].clone(),
            &color_pickers,
            &movers,
            bounds,
            seed,
        );  
        emitters.push(emitter); 
    }


    Model { emitters }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    let _t = _app.elapsed_frames() as f32 / 360.;
    for e in _model.emitters.iter_mut() {
        e.update(_t);
    }
    for e in _model.emitters.iter_mut() {
        if random_f32() > 0.9 {
            e.emit();
            
            // conditionally apply force that changes over time
            // e.apply_force(vec2(0. * 0.004, -1. * 0.002));
            // e.apply_force(vec2(_t.cos() * 0.005, _t.sin() * 0.005));
            // e.apply_force(vec2(-1. * 0.004, 1. * 0.002));
        }
    }
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    // Begin drawing
    let draw = _app.draw();

    if _app.elapsed_frames() == 0 {
        draw.background().color(BLACK);
    }

    // Draw the emitters
    for e in _model.emitters.iter() {
        e.display(&draw);
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(_app, &frame).unwrap();
}

fn key_released(_app: &App, _model: &mut Model, key: Key) {
    trace!("{:?}", key);
    match key {
        Key::Space => {
            info!("Toggling pause");
            for e in _model.emitters.iter_mut() {
                e.toggle_pause();
            }
        }
        Key::S => {
            let file_path = captured_frame_path(_app);
            info!("Capturing frame to {:?}", file_path);
            _app.main_window().capture_frame(file_path);
        }
        Key::M => {
            info!("Mouse Position: {:#?}", _app.mouse.position());
        }
        _ => {}
    }
}

fn captured_frame_path(_app: &App) -> std::path::PathBuf {
    _app.project_path()
        .expect("failed to locate `project_path`")
        .join("frames")
        .join("out")
        .with_extension("png")
}

fn main() {
    pretty_env_logger::init();
    // info!("{:?}", *OPT);
    nannou::app(model).update(update).run();
}
