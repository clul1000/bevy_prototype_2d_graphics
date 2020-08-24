use bevy::{
    render::{
        pipeline::{
            PipelineSpecialization, PipelineDescriptor, RenderPipeline,
            DynamicBinding, PrimitiveTopology
        },
        shader::{ShaderStage, ShaderStages},
        renderer::{RenderResource, RenderResources},
        render_graph::{RenderGraph, base, RenderResourcesNode}, mesh::VertexAttribute
    },
    core::Byteable,
    prelude::*, math::Mat2,
};
use std::ops::{DerefMut, Deref};
use super::Immediate2DGraphics;
use base::MainPass;

#[derive(Clone, Debug, RenderResources, RenderResource)]
#[render_resources(from_self)]
#[allow(unused)]
pub struct LineStyle {
    pub color: Color,
    pub width: f32,
    pub height: f32,
    pub stroke: f32,
}
unsafe impl Byteable for LineStyle {}

impl Default for LineStyle {
    fn default() -> Self {
        Self {
            color: Color::BLACK,
            width: 0.,
            height: 0.,
            stroke: 10.,
        }
    }
}

#[derive(Debug)]
pub struct Line {
    pub style: LineStyle,
    pub start: Vec2,
    pub stop: Vec2,
}

impl Line {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self {
            style: LineStyle::default(),
            start: Vec2::new(x1, y1),
            stop: Vec2::new(x2, y2),
        }
    }
}

pub struct LineBuilder<'a> {
    pub(crate) graphics: &'a mut Immediate2DGraphics,
}

impl<'a> LineBuilder<'a> {
    pub fn with_stroke(self, stroke: f32) -> Self {
        let line = self.graphics.lines.last_mut().unwrap();
        line.style.stroke = stroke;
        self
    }

    pub fn with_color(self, color: Color) -> Self {
        let line = self.graphics.lines.last_mut().unwrap();
        line.style.color = color;
        self
    }
}

impl<'a> Deref for LineBuilder<'a> {
    type Target = Immediate2DGraphics;
    fn deref(&self) -> &Self::Target {
        &*self.graphics
    }
}

impl<'a> DerefMut for LineBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.graphics
    }
}

impl<'a> Drop for LineBuilder<'a> {
    fn drop(&mut self) {
        let mut line = self.graphics.lines.last_mut().unwrap();
        let len = (line.start - line.stop).length();
        let width = len + 2. * line.style.stroke;
        let height = 2. * line.style.stroke;
        line.style.height = height;
        line.style.width = width;
    }
}

const LINE_PIPELINE: Handle<PipelineDescriptor> = Handle::from_u128(67859415639327501719432658702909922763);

const LINE_STYLE_NODE: &str = "LineStyle";

pub fn add_render_graph(resources: &Resources) {
        let mut render_graph = resources.get_mut::<RenderGraph>().unwrap();

        let mut shaders = resources.get_mut::<Assets<Shader>>().unwrap();

        let mut pipelines = resources.get_mut::<Assets<PipelineDescriptor>>().unwrap();

        let resource_node = RenderResourcesNode::<LineStyle>::new(true);
        render_graph.add_system_node(LINE_STYLE_NODE, resource_node);
        render_graph.add_node_edge(LINE_STYLE_NODE, base::node::MAIN_PASS).unwrap();

        let vertex_shader_handle = shaders.add(
            Shader::from_glsl(ShaderStage::Vertex, include_str!("shader/line.vert"))
        );
        let fragment_shader_handle = shaders.add(
            Shader::from_glsl(ShaderStage::Fragment, include_str!("shader/line.frag"))
        );
        let pipeline_descriptor = PipelineDescriptor::default_config(ShaderStages {
            vertex: vertex_shader_handle,
            fragment: Some(fragment_shader_handle),
        });

        pipelines.set(LINE_PIPELINE, pipeline_descriptor);

        let mesh = Mesh {
            primitive_topology: PrimitiveTopology::TriangleList,
            attributes: vec![
                VertexAttribute::position(vec![
                    [0., 0., 0.],
                    [1., 1., 0.],
                    [0., 1., 0.],
                    [1., 0., 0.],
                ]),
                VertexAttribute::normal(vec![
                    [0., 0., 1.],
                    [0., 0., 1.],
                    [0., 0., 1.],
                    [0., 0., 1.],
                ]),
                VertexAttribute::uv(vec![
                    [0., 0.],
                    [1., 1.],
                    [0., 1.],
                    [1., 0.],
                ])
            ],
            indices: Some(vec![
                0, 1, 2,
                0, 3, 1,
            ]),
        };

        let mut mesh_assets = resources.get_mut::<Assets<Mesh>>().unwrap();
        mesh_assets.set(LINE_QUAD_HANDLE, mesh);
}

const LINE_QUAD_HANDLE: Handle<Mesh> = Handle::from_u128(39274529312965987326587436587346338379);

#[derive(Default, Bundle)]
struct LineComponents {
    pub mesh: Handle<Mesh>,
    pub draw: Draw,
    pub render_pipelines: RenderPipelines,
    pub main_pass: MainPass,
    pub transform: Transform,
}

fn calc_transformation_matrix(from: [Vec2; 4], to: [Vec2; 4]) -> Mat4 {
    // ToMat = M * FromMat
    // ToMat * FromMat^-1 = M
    // Using last coordinate to verify result.

    let from_mat = Mat3::from_cols(
        Vec3::new(from[0].x(), from[0].y(), 1.),
        Vec3::new(from[1].x(), from[1].y(), 1.),
        Vec3::new(from[2].x(), from[2].y(), 1.),
    );

    let to_mat = Mat3::from_cols(
        Vec3::new(to[0].x(), to[0].y(), 1.),
        Vec3::new(to[1].x(), to[1].y(), 1.),
        Vec3::new(to[2].x(), to[2].y(), 1.),
    );

    let result = to_mat * from_mat.inverse();

    assert!(result.z_axis().z() - 1. <= f32::EPSILON);

    let from_test = Vec3::new(from[3].x(), from[3].y(), 1.);
    let to_test = Vec3::new(to[3].x(), to[3].y(), 1.);

    let len = (result * from_test - to_test).length_squared();

    assert!(len <= f32::EPSILON);

    // result:
    // a b t
    // c d p
    // 0 0 1
    //
    // return
    // a b 0 t
    // c d 0 p
    // 0 0 0 0
    // 0 0 0 1

    let x_axis = Vec4::new(result.x_axis().x(), result.x_axis().y(), 0., 0.);
    let y_axis = Vec4::new(result.y_axis().x(), result.y_axis().y(), 0., 0.);
    let z_axis = Vec4::new(0., 0., 0., 0.);
    let w_axis = Vec4::new(result.z_axis().x(), result.z_axis().y(), 0., 1.);
    Mat4::from_cols(x_axis, y_axis, z_axis, w_axis)
}

fn calc_transform_for_line(start: Vec2, end: Vec2, stroke: f32) -> Mat4 {
    let from = [
        Vec2::new(0., 0.),
        Vec2::new(1., 1.),
        Vec2::new(0., 1.),
        Vec2::new(1., 0.),
    ];

    // calculate line bounding box.

    let dir = (end - start).normalize() * stroke;

    // to_2   X                        X       to_1
    //        |                        |
    // a ---start-dir->--- ... ---------end----- b
    //        |                        |
    // to_0   X                        X       to_3

    let ccw_rotation = Mat2::from_cols(Vec2::new(0., 1.), Vec2::new(-1., 0.)); //CW
    let cw_rotation = Mat2::from_cols(Vec2::new(0., -1.), Vec2::new(1., 0.)); //CCW

    let a = start - dir;
    let to_0 = a + cw_rotation * dir;
    let to_2 = a + ccw_rotation * dir;

    let b = end + dir;
    let to_1 = b + ccw_rotation * dir;
    let to_3 = b + cw_rotation * dir;

    let to = [to_0, to_1, to_2, to_3];

    calc_transformation_matrix(from, to)
}

pub fn line_update_system(
    mut commands: Commands,
    mut immediate_graphics: ResMut<Immediate2DGraphics>,
    mut query: Query<(
        &mut Draw,
        &mut LineStyle,
        &mut Transform,
    )>,
) {
    commands.spawn(Camera2dComponents::default());

    let line_render_pipelines = || // Workaround because RenderPipelines is not Clone.
        RenderPipelines::from_pipelines(vec![RenderPipeline::specialized(
            LINE_PIPELINE,
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

    let mut lines_iter = immediate_graphics.lines.drain(..);

    for (mut draw, mut style, mut transform) in &mut query_iter {
        let line = if let Some(line) = lines_iter.next() {
            line
        } else {
            draw.is_visible = false;
            break;
        };

        draw.is_visible = true;
        *style = line.style.clone();
        *transform = Transform::new(calc_transform_for_line(line.start, line.stop, line.style.stroke));
    }

    for (mut draw, _, _) in query_iter {
        draw.is_visible = false;
    }

    for line in lines_iter {
        commands.spawn(LineComponents {
            mesh: LINE_QUAD_HANDLE,
            render_pipelines: line_render_pipelines(),
            transform: Transform::new(calc_transform_for_line(line.start, line.stop, line.style.stroke)),
            ..Default::default()
        })
        .with(line.style);
    }
}

