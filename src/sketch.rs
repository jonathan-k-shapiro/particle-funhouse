use nannou::{geom::rect, prelude::*};
use log::*;
use particle_lib::mover::{epicycloid, p_elipse};

use particle_lib::config::MoverConfig;
use particle_lib::mover::Mover;


fn main() {
    pretty_env_logger::init();
    nannou::sketch(view).size(1600, 1600).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect().w_h();
    let w_h = vec2(win.0, win.1);
    let outer = vec2(0.3, 0.4);
    let inner = vec2(1., 3.);
    let translate = vec2(-200., 200.);
    let index = app.time / 10.0;

    let config = MoverConfig {
        mover_type: "p_elipse".to_string(),
        inner,
        outer,
        scale: w_h,
        translation: Some(translate),
        rotation_angle: Some(0.),
        rotation_speed: Some(0.),
    };  
    let mover = Mover::from_config("test_mover".to_string(), config);    
    let position = mover.get_postion(index);

    // let position = p_mover::epicycloid(index, -3., 3., 10., 3.3333, vec2(10., 10.));
    // let position = p_mover::p_trig(
    //     index,
    //     -600.0,
    //     600.0,
    //     vec2(1., 3.),  //inner
    //     vec2(2., 1.), //outer
    //     p_mover::test_transform_2,
    //     vec2(20., 50.),
    // );

    if app.elapsed_frames() == 0 {
        frame.clear(BLACK);
    }
    trace!("{:?} {:?}", index, position);
    draw.ellipse()
        .xy(position)
        // .stroke_weight(2.)
        .color(WHITE)
        .w_h(4.0, 4.0);

    draw.to_frame(app, &frame).unwrap();
}
