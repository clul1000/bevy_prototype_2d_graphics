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

#[derive(Debug, RenderResources, RenderResource)]
#[render_resources(from_self)]
#[allow(unused)]
pub struct CircleStyle {
    pub fill_color: Color,
    pub border_color: Color,
    pub border_width: f32,
}
unsafe impl Byteable for CircleStyle {}

impl Default for CircleStyle {
    fn default() -> Self {
        Self {
            fill_color: Color::BLACK,
            border_color: Color::BLACK,
            border_width: 0.,
        }
    }
}

#[derive(Debug)]
pub struct Circle {
    pub style: CircleStyle,
    pub pos: Vec2,
    pub radius: f32,
}

impl Circle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            style: CircleStyle::default(),
            pos: Vec2::new(x, y),
            radius: 100.,
        }
    }
}

pub struct CircleBuilder<'a> {
    pub(crate) graphics: &'a mut Immediate2DGraphics,
}

impl<'a> CircleBuilder<'a> {
    /// Define radius of circle.
    pub fn with_radius(self, radius: f32) -> Self {
        let circle = self.graphics.circles.last_mut().unwrap();
        circle.radius = radius;
        self
    }

    /// Define color of circle.
    pub fn with_color(self, color: Color) -> Self {
        let circle = self.graphics.circles.last_mut().unwrap();
        circle.style.fill_color = color;
        self
    }

    /// Add border to circle.
    /// If stroke is 0, no border is visible.
    /// If stroke is 1, the border covers the entire circle.
    pub fn with_border(self, color: Color, stroke: f32) -> Self {
        let circle = self.graphics.circles.last_mut().unwrap();
        circle.style.border_color = color;
        circle.style.border_width = stroke;
        self
    }
}

impl<'a> Deref for CircleBuilder<'a> {
    type Target = Immediate2DGraphics;
    fn deref(&self) -> &Self::Target {
        &*self.graphics
    }
}

impl<'a> DerefMut for CircleBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.graphics
    }
}

const CIRCLE_PIPELINE: Handle<PipelineDescriptor> = Handle::from_u128(324098732123374799960298734098274887483);

const CIRCLE_STYLE_NODE: &str = "CircleStyle";

pub fn add_render_graph(resources: &Resources) {
        let mut render_graph = resources.get_mut::<RenderGraph>().unwrap();

        let mut shaders = resources.get_mut::<Assets<Shader>>().unwrap();

        let mut pipelines = resources.get_mut::<Assets<PipelineDescriptor>>().unwrap();

        let resource_node = RenderResourcesNode::<CircleStyle>::new(true);
        render_graph.add_system_node(CIRCLE_STYLE_NODE, resource_node);
        render_graph.add_node_edge(CIRCLE_STYLE_NODE, base::node::MAIN_PASS).unwrap();

        let vertex_shader_handle = shaders.add(
            Shader::from_glsl(ShaderStage::Vertex, include_str!("shader/circle.vert"))
        );
        let fragment_shader_handle = shaders.add(
            Shader::from_glsl(ShaderStage::Fragment, include_str!("shader/circle.frag"))
        );
        let pipeline_descriptor = PipelineDescriptor::default_config(ShaderStages {
            vertex: vertex_shader_handle,
            fragment: Some(fragment_shader_handle),
        });

        pipelines.set(CIRCLE_PIPELINE, pipeline_descriptor);

}

pub fn circle_update_system(
    mut commands: Commands,
    mut immediate_graphics: ResMut<Immediate2DGraphics>,
    mut query: Query<(
        &mut Draw, 
        &mut CircleStyle, 
        &mut Translation,
        &mut Scale,
    )>,
) {
    commands.spawn(Camera2dComponents::default());

    let circle_render_pipelines = || // Workaround because RenderPipelines is not Clone.
        RenderPipelines::from_pipelines(vec![RenderPipeline::specialized(
            CIRCLE_PIPELINE,
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

    let mut circle_iter = immediate_graphics.circles.drain(..);

    for (mut draw, mut style, mut trans, mut scale) in &mut query_iter {
        let circle = if let Some(circle) = circle_iter.next() {
            circle
        } else {
            draw.is_visible = false;
            break;
        };

        draw.is_visible = true;
        *style = circle.style;
        *scale = Scale::from(circle.radius * 2.);
        *trans = Translation::from(circle.pos.extend(0.));
    }

    for (mut draw, _, _, _) in query_iter {
        draw.is_visible = false;
    }

    for circle in circle_iter {
        commands.spawn(MeshComponents {
            mesh: QUAD_HANDLE,
            render_pipelines: circle_render_pipelines(),
            scale: Scale::from(circle.radius * 2.),
            translation: Translation::from(circle.pos.extend(0.)),
            ..Default::default()
        })
        .with(circle.style);
    }
}

