use bevy::{
    render::{
        pipeline::{
            PipelineSpecialization, PipelineDescriptor, RenderPipeline,
            DynamicBinding
        },
        shader::{ShaderStage, ShaderStages},
        renderer::{RenderResource, RenderResources},
        render_graph::{RenderGraph, base, RenderResourcesNode}
    },
    core::Byteable,
    sprite::QUAD_HANDLE,
    prelude::*,
};
use std::ops::{DerefMut, Deref};
use super::Immediate2DGraphics;
use base::MainPass;

#[derive(Debug, RenderResources, RenderResource)]
#[render_resources(from_self)]
#[allow(unused)]
pub struct RectangleStyle {
    pub fill_color: Color,
    pub border_color: Color,
    pub border_width: Vec2,
}
unsafe impl Byteable for RectangleStyle {}

impl Default for RectangleStyle {
    fn default() -> Self {
        Self {
            fill_color: Color::BLACK,
            border_color: Color::BLACK,
            border_width: Vec2::zero(),
        }
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub style: RectangleStyle,
    pub pos: Vec2,
    pub dimensions: Vec2,
    pub rotation: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            style: RectangleStyle::default(),
            pos: Vec2::new(x, y),
            dimensions: Vec2::new(100., 100.),
            rotation: 0.
        }
    }
}

pub struct RectangleBuilder<'a> {
    pub(crate) graphics: &'a mut Immediate2DGraphics,
}

impl<'a> RectangleBuilder<'a> {
    /// Define color of circle.
    pub fn with_color(self, color: Color) -> Self {
        let rectangle = self.graphics.rectangles.last_mut().unwrap();
        rectangle.style.fill_color = color;
        self
    }

    /// Add border to circle.
    /// If stroke is 0, no border is visible.
    /// If stroke is 1, the border covers the entire circle.
    pub fn with_border(self, color: Color, stroke: f32) -> Self {
        let rectangles = self.graphics.rectangles.last_mut().unwrap();
        rectangles.style.border_color = color;
        rectangles.style.border_width = Vec2::new(stroke, stroke);
        self
    }

    pub fn with_rotation(self, rotation: f32) -> Self {
        let rectangles = self.graphics.rectangles.last_mut().unwrap();
        rectangles.rotation = rotation;
        self
    }

    pub fn with_width(self, width: f32) -> Self {
        let rectangles = self.graphics.rectangles.last_mut().unwrap();
        rectangles.dimensions.set_x(width);
        self
    }

    pub fn with_height(self, height: f32) -> Self {
        let rectangles = self.graphics.rectangles.last_mut().unwrap();
        rectangles.dimensions.set_y(height);
        self
    }
}

impl<'a> Drop for RectangleBuilder<'a> {
    fn drop(&mut self) {
        let rectangles = self.graphics.rectangles.last_mut().unwrap();
        let width = rectangles.dimensions.x();
        let height = rectangles.dimensions.y();

        // border widths have to adjusted on non-square rectangles, because the shader assumes
        // the uvs to be a square.
        if width > height {
            let ratio = height / width;
            *rectangles.style.border_width.x_mut() *= ratio;
        } else {
            let ratio = width / height;
            *rectangles.style.border_width.y_mut() *= ratio;
        }
    }
}

impl<'a> Deref for RectangleBuilder<'a> {
    type Target = Immediate2DGraphics;
    fn deref(&self) -> &Self::Target {
        &*self.graphics
    }
}

impl<'a> DerefMut for RectangleBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.graphics
    }
}

const RECTANGLE_PIPELINE: Handle<PipelineDescriptor> = Handle::from_u128(309579415678454561098776985274718569653);

const RECTANGLE_STYLE_NODE: &str = "RectangleStyle";

pub fn add_render_graph(resources: &Resources) {
        let mut render_graph = resources.get_mut::<RenderGraph>().unwrap();

        let mut shaders = resources.get_mut::<Assets<Shader>>().unwrap();

        let mut pipelines = resources.get_mut::<Assets<PipelineDescriptor>>().unwrap();

        let resource_node = RenderResourcesNode::<RectangleStyle>::new(true);
        render_graph.add_system_node(RECTANGLE_STYLE_NODE, resource_node);
        render_graph.add_node_edge(RECTANGLE_STYLE_NODE, base::node::MAIN_PASS).unwrap();

        let vertex_shader_handle = shaders.add(
            Shader::from_glsl(ShaderStage::Vertex, include_str!("shader/rectangle.vert"))
        );
        let fragment_shader_handle = shaders.add(
            Shader::from_glsl(ShaderStage::Fragment, include_str!("shader/rectangle.frag"))
        );
        let pipeline_descriptor = PipelineDescriptor::default_config(ShaderStages {
            vertex: vertex_shader_handle,
            fragment: Some(fragment_shader_handle),
        });

        pipelines.set(RECTANGLE_PIPELINE, pipeline_descriptor);

}

#[derive(Default, Bundle)]
struct RectangleComponents {
    pub mesh: Handle<Mesh>,
    pub draw: Draw,
    pub render_pipelines: RenderPipelines,
    pub main_pass: MainPass,
    pub transform: Transform,
    pub translation: Translation,
    pub rotation: Rotation,
    pub scale: NonUniformScale,
}

pub fn rectangle_update_system(
    mut commands: Commands,
    mut immediate_graphics: ResMut<Immediate2DGraphics>,
    mut query: Query<(
        &mut Draw,
        &mut RectangleStyle,
        &mut Translation,
        &mut NonUniformScale,
        &mut Rotation,
    )>,
) {
    commands.spawn(Camera2dComponents::default());

    let rectangle_render_pipelines = || // Workaround because RenderPipelines is not Clone.
        RenderPipelines::from_pipelines(vec![RenderPipeline::specialized(
            RECTANGLE_PIPELINE,
            PipelineSpecialization {
                dynamic_bindings: vec![
                    DynamicBinding {
                        bind_group: 1,
                        binding: 0,
                    },
                    DynamicBinding {
                        bind_group: 1,
                        binding: 1,
                    },
                ],
                ..Default::default()
            },
            )
        ]);

    let mut query_borrow = query.iter();
    let mut query_iter = query_borrow.iter();

    let mut rectangle_iter = immediate_graphics.rectangles.drain(..);

    for (mut draw, mut style, mut trans, mut scale, mut rot) in &mut query_iter {
        let rectangle = if let Some(rectangle) = rectangle_iter.next() {
            rectangle
        } else {
            draw.is_visible = false;
            break;
        };

        draw.is_visible = true;
        *style = rectangle.style;
        *scale = NonUniformScale::new(rectangle.dimensions.x(), rectangle.dimensions.y(), 0.);
        *trans = Translation::from(rectangle.pos.extend(0.));
        *rot = Rotation::from_rotation_z(rectangle.rotation);
    }

    for (mut draw, _, _, _, _) in query_iter {
        draw.is_visible = false;
    }

    for rectangle in rectangle_iter {
        commands.spawn(RectangleComponents {
            mesh: QUAD_HANDLE,
            render_pipelines: rectangle_render_pipelines(),
            scale: NonUniformScale::new(rectangle.dimensions.x(), rectangle.dimensions.y(), 0.),
            translation: Translation::from(rectangle.pos.extend(0.)),
            rotation: Rotation::from_rotation_z(rectangle.rotation),
            ..Default::default()
        })
        .with(rectangle.style);
    }
}

