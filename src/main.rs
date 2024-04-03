extern crate nannou;

use nannou::prelude::*;
use lazy_static::lazy_static;
use log::*;

pub mod particle;
pub mod emitter;
pub mod color_picker;
pub mod config;

lazy_static! { 
    #[derive(Debug)]
    pub static ref CONFIG: config::Config = config::read_config("config.toml");
}

struct Model {
    emitter: emitter::Emitter,
}

fn model(_app: &App) -> Model {
    _app.new_window()
        .size(600,600)
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

    // let emitter = emitter::Emitter::with_noise_field(bounds, |bounds: emitter::Bounds| {
    //     let w = bounds.right - bounds.left;
    //     let h = bounds.top - bounds.bottom;
    //     let pos = pt2(
    //         ((random_f32() * 2. - 1.) * w / 2.).floor(), 
    //         ((random_f32() * 2. - 1.) * h / 2.).floor(),
    //     );
    //     let vel = vec2(0., 0.); // vec2(random_f32() * 2.0 - 1.0, random_f32() - 1.0);
    //     let hue = 0.5; // random_f32() * 360.0;
    //     let radius = 10.0;
    //     let life_span = 512.0;
    //     particle::Particle::new(pos, vel, hue, radius, life_span)
    // });
    // let mut emitter = emitter::Emitter::new(bounds);
    let emitter = emitter::Emitter::from_config(CONFIG.emitters.as_ref().unwrap()["emitter_1"].clone(), bounds);
    Model {
        emitter,
    }
}

fn update(_app: &App, _m: &mut Model, _update: Update) {
    let _t = _app.elapsed_frames() as f32 / 360.;
    _m.emitter.update();
    // if _app.elapsed_frames() < 100 && random_f32() > 0.9 {
    if random_f32() > 0.9 {
        _m.emitter.emit();
        // _m.emitter.apply_force(vec2(_t.cos() * 0.005, _t.sin() * 0.005));
    }
}

fn view(_app: &App, _m: &Model, frame: Frame){
   // Begin drawing
   let draw = _app.draw();

   if _app.elapsed_frames() == 0 {
       draw.background().color(BLACK);
   }

    // Draw the emitter
    _m.emitter.display(&draw);

   // Write the result of our drawing to the window's frame.
   draw.to_frame(_app, &frame).unwrap();

}

fn key_released(_app: &App, _model: &mut Model, key: Key) {
    trace!("{:?}", key);
    match key {
        Key::Space => {
            info!("Toggling pause");
            _model.emitter.toggle_pause();
        },
        Key::S => {
            let file_path = captured_frame_path(_app);
            info!("Capturing frame to {:?}", file_path);
            _app.main_window().capture_frame(file_path);
        },
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
