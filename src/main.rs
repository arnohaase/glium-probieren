#[macro_use]
extern crate glium;

use glium::{Surface, DrawParameters, PolygonMode, BackfaceCullingMode};
use cgmath::*;
use glium::uniforms::{AsUniformValue, UniformValue};
use std::intrinsics::transmute;


struct GlslMatrix4 <'a> {
    m: &'a Matrix4<f32>
}
impl <'a> GlslMatrix4 <'a> {
    pub fn of(m: &'a Matrix4<f32>) -> GlslMatrix4 {
        GlslMatrix4 {m}
    }
}
impl <'a> AsUniformValue for GlslMatrix4<'a> {
    fn as_uniform_value(&self) -> UniformValue {
        //TODO avoid copying?
        UniformValue::Mat4(
            [
                [self.m.x.x, self.m.x.y, self.m.x.z, self.m.x.w],
                [self.m.y.x, self.m.y.y, self.m.y.z, self.m.y.w],
                [self.m.z.x, self.m.z.y, self.m.z.z, self.m.z.w],
                [self.m.w.x, self.m.w.y, self.m.w.z, self.m.w.w]
            ]
        )
    }
}



fn main() {

    use glium::glutin;

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    #[derive(Copy, Clone, Debug)]
    struct Vertex {
        position: [f32; 2],
    }
    implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let mut shape = vec![vertex1, vertex2, vertex3];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        out vec2 my_attr;
        uniform mat4 matrix;
        void main() {
            my_attr = position;
            gl_Position = matrix*vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        in vec2 my_attr;
        out vec4 color;
        void main() {
            color = vec4(my_attr, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let start = std::time::Instant::now();

    event_loop.run(move |ev, _, control_flow| {

        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        // *control_flow = glutin::event_loop::ControlFlow::Poll;

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }

        let elapsed_millis = std::time::Instant::now().duration_since(start).as_millis() as u64;

        let t = (elapsed_millis % 1000) as f32 / 1000.0 - 0.5;

        let mut target = display.draw();
        target.clear_color(0., 0., 1., 1.);

        let draw_parameters: DrawParameters = DrawParameters {
            // polygon_mode: PolygonMode::Line,
            // line_width: Some(1.0),
            // backface_culling: BackfaceCullingMode::CullCounterClockwise,
            ..Default::default()
        };

        let matrix = Matrix4::from_translation(Vector3::unit_x() * t);
        let matrix = matrix * Matrix4::from_angle_z(Rad { 0: t});

        target.draw(&vertex_buffer,
                    &indices, &program,
                    &uniform! {matrix: GlslMatrix4::of(&matrix)},
                    // &uniform! {matrix: as_uniform_value(&matrix)},
                    &draw_parameters
        ).unwrap();

        target.finish().unwrap();

    });
}
