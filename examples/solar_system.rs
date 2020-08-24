use bevy::prelude::*;
use bevy_prototype_2d_graphics::{Immediate2DGraphics, Immediate2DGraphicsPlugin};

fn drawing_solar_system(
    time: Res<Time>,
    mut graphics: ResMut<Immediate2DGraphics>
) {
    let x = f32::sin(time.seconds_since_startup as f32) * 300.;
    let y = f32::cos(time.seconds_since_startup as f32) * 300.;

    let rot = time.seconds_since_startup as f32;

    graphics
        .fill_circle(0., 0.)
        .with_radius(100.)
        .with_color(Color::rgb(1., 1., 0.))
        .with_border(Color::rgb(0.9, 0.3, 0.), 0.2)
        .fill_rectangle(x, y)
        .with_width(100.)
        .with_height(50.)
        .with_color(Color::GREEN)
        .with_border(Color::BLUE, 0.4)
        .with_rotation(rot);
}

fn main() {
    let mut app = App::build();
    app
        .add_default_plugins()
        .add_plugin(Immediate2DGraphicsPlugin)
        .add_system(drawing_solar_system.system())
        .run();

}
