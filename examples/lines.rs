use bevy::prelude::*;
use bevy_prototype_2d_graphics::{Immediate2DGraphics, Immediate2DGraphicsPlugin};

fn drawing_solar_system(
    time: Res<Time>,
    mut graphics: ResMut<Immediate2DGraphics>
) {
    let rot = time.seconds_since_startup as f32;

    graphics
        .draw_line(200., -100., 200., 200.)
        .with_stroke(f32::sin(rot * 1.2) * 10. + 10.)
        .with_color(Color::rgb(0., 5., 2.));

    graphics
        .draw_line(-200., -100., -200., 200.)
        .with_stroke(f32::sin(rot) * 10. + 10.)
        .with_color(Color::rgb(1., 0., 2.));

}

fn main() {
    let mut app = App::build();
    app
        .add_default_plugins()
        .add_plugin(Immediate2DGraphicsPlugin)
        .add_system(drawing_solar_system.system())
        .run();
}
