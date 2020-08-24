use bevy::prelude::*;

use circles::{CircleBuilder, Circle};
use rectangles::{RectangleBuilder, Rectangle};
use lines::{LineBuilder, Line};

mod circles;
mod rectangles;
mod lines;

#[derive(Debug, Default)]
pub struct Immediate2DGraphics {
    circles: Vec<Circle>,
    rectangles: Vec<Rectangle>,
    lines: Vec<Line>,
}

impl Immediate2DGraphics {
    pub fn fill_circle(&mut self, x: f32, y: f32) -> CircleBuilder<'_> {
        self.circles.push(Circle::new(x, y));

        CircleBuilder {
            graphics: self
        }
    }

    pub fn fill_rectangle(&mut self, x: f32, y: f32) -> RectangleBuilder<'_> {
        self.rectangles.push(Rectangle::new(x, y));

        RectangleBuilder {
            graphics: self
        }
    }

    pub fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) -> LineBuilder<'_> {
        self.lines.push(Line::new(x1, y1, x2, y2));

        LineBuilder {
            graphics: self
        }
    }
}

pub struct Immediate2DGraphicsPlugin;

impl Plugin for Immediate2DGraphicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Immediate2DGraphics>()
            // It only works if stage is UPDATE, POST_UPDATE only works sometimes.
            // There has to be some system that interferes with there.
            .add_system_to_stage(stage::UPDATE, circles::circle_update_system.system())
            .add_system_to_stage(stage::UPDATE, rectangles::rectangle_update_system.system())
            .add_system_to_stage(stage::UPDATE, lines::line_update_system.system());

        let resources = app.resources();

        circles::add_render_graph(&*resources);
        rectangles::add_render_graph(&*resources);
        lines::add_render_graph(&*resources);
    }
}
