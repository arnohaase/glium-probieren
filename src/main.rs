#[macro_use]
extern crate glium;

use glium::{Surface, DrawParameters, PolygonMode, BackfaceCullingMode};

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
        #version 460
        in vec2 position;
        out vec2 my_attr;
        uniform float t;
        void main() {
            vec2 pos = position;
            my_attr = position;
            pos.x += t;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 460
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

        let offs_x = (elapsed_millis % 1000) as f32 / 1000.0 - 0.5;

        let mut target = display.draw();
        target.clear_color(0., 0., 1., 1.);

        let draw_parameters: DrawParameters = DrawParameters {
            // polygon_mode: PolygonMode::Line,
            // line_width: Some(1.0),
            // backface_culling: BackfaceCullingMode::CullCounterClockwise,
            ..Default::default()
        };
        target.draw(&vertex_buffer, &indices, &program, &uniform! {t: offs_x}, &draw_parameters).unwrap();

        target.finish().unwrap();

    });
}
