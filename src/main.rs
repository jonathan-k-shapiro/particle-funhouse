extern crate nannou;

use nannou::{color::named, prelude::*};
use log::*;

pub mod particle;
pub mod emitter;

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

    let r = _app.window_rect().right() as f32;
    let l = _app.window_rect().left() as f32;

    let t = _app.window_rect().top() as f32;
    let b = _app.window_rect().bottom() as f32;
    let bounds: emitter::Bounds = emitter::Bounds {
        top: t,
        bottom: b,
        left: l,
        right: r,
    };

    let emitter = emitter::Emitter::with_noise_field(bounds, |bounds: emitter::Bounds| {
        let w = bounds.right - bounds.left;
        let h = bounds.top - bounds.bottom;
        let pos = pt2(
            ((random_f32() * 2. - 1.) * w / 2.).floor(), 
            ((random_f32() * 2. - 1.) * h / 2.).floor(),
        );
        let vel = vec2(0., 0.); // vec2(random_f32() * 2.0 - 1.0, random_f32() - 1.0);
        let hue = random_f32() * 360.0;
        let radius = 10.0;
        let life_span = 512.0;
        particle::Particle::new(pos, vel, hue, radius, life_span)
    });
    Model {
        emitter,
    }
}

fn update(_app: &App, _m: &mut Model, _update: Update) {
    let t = _app.elapsed_frames() as f32 / 360.;
    _m.emitter.update();
    // if _app.elapsed_frames() < 100 && random_f32() > 0.9 {
    if random_f32() > 0.9 {
        _m.emitter.emit();
        // _m.emitter.apply_force(vec2(t.cos() * 0.005, t.sin() * 0.005));
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
