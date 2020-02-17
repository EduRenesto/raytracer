use vek::vec::repr_c::{rgb::Rgb, vec2::Vec2, vec3::Vec3};

use super::gl_shader::GlShader;
use super::gl_texture::GlTexture;

type Receiver = std::sync::mpsc::Receiver<(Vec2<usize>, Rgb<f32>)>;

/// A simple OpenGL-based realtime visualizer
pub struct Visualizer {
    receiver: Receiver,
    w: usize,
    h: usize,
    event_loop: glutin::event_loop::EventLoop<()>,
}

impl Visualizer {
    pub fn new(mut receiver: Receiver, w: usize, h: usize) -> Visualizer {
        let event_loop = glutin::event_loop::EventLoop::new();

        let builder = glutin::window::WindowBuilder::new()
            .with_title("raytracer")
            .with_inner_size((w as u32, h as u32).into());

        let ctx = glutin::ContextBuilder::new()
            .build_windowed(builder, &event_loop)
            .unwrap();

        let ctx = unsafe { ctx.make_current().unwrap() };

        gl::load_with(|s| ctx.get_proc_address(s) as *const std::ffi::c_void);

        let mut vao = 0 as gl::types::GLuint;

        {
            let mut pos_vbo = 0 as gl::types::GLuint;
            let mut uvs_vbo = 0 as gl::types::GLuint;

            let positions = vec![
                Vec3::new(1.0f32, 1.0f32, 0.0f32),   // top right
                Vec3::new(-1.0f32, 1.0f32, 0.0f32),  // top left
                Vec3::new(-1.0f32, -1.0f32, 0.0f32), // bottom left
                Vec3::new(-1.0f32, -1.0f32, 0.0f32), // bottom left
                Vec3::new(1.0f32, -1.0f32, 0.0f32),  // bottom right
                Vec3::new(1.0f32, 1.0f32, 0.0f32),   // top right
            ];

            let uvs = vec![
                Vec2::new(0.0f32, 0.0f32),
                Vec2::new(1.0f32, 0.0f32),
                Vec2::new(1.0f32, 1.0f32),
                Vec2::new(1.0f32, 1.0f32),
                Vec2::new(0.0f32, 1.0f32),
                Vec2::new(0.0f32, 0.0f32),
            ];

            unsafe {
                gl::GenVertexArrays(1, &mut vao);
                gl::BindVertexArray(vao);

                gl::GenBuffers(1, &mut pos_vbo);
                gl::GenBuffers(1, &mut uvs_vbo);

                gl::BindBuffer(gl::ARRAY_BUFFER, pos_vbo);
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    12 * positions.len() as isize,
                    positions.as_ptr() as *const gl::types::GLvoid,
                    gl::STATIC_DRAW,
                );
                gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::TRUE, 0, std::ptr::null());
                gl::EnableVertexAttribArray(0);

                gl::BindBuffer(gl::ARRAY_BUFFER, uvs_vbo);
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    8 * uvs.len() as isize,
                    uvs.as_ptr() as *const gl::types::GLvoid,
                    gl::STATIC_DRAW,
                );
                gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::TRUE, 0, std::ptr::null());
                gl::EnableVertexAttribArray(1);
            }
        }

        let tex = GlTexture::new(w, h, None);

        let shader = GlShader::new(vec![
            Box::new((gl::VERTEX_SHADER, "res/show_texture.vs.glsl".to_string())),
            Box::new((gl::FRAGMENT_SHADER, "res/show_texture.fs.glsl".to_string())),
        ])
        .unwrap();

        event_loop.run(move |evt, _, control_flow| {
            *control_flow = glutin::event_loop::ControlFlow::Poll;

            match evt {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit
                    }
                    glutin::event::WindowEvent::RedrawRequested => {
                        Visualizer::show_image(vao, &tex, &shader, &mut receiver);
                        ctx.swap_buffers().unwrap();
                    }
                    _ => *control_flow = { glutin::event_loop::ControlFlow::Poll },
                },
                _ => {
                    Visualizer::show_image(vao, &tex, &shader, &mut receiver);
                    ctx.swap_buffers().unwrap();
                    *control_flow = glutin::event_loop::ControlFlow::Poll;
                }
            }
        });

        Visualizer {
            receiver,
            w,
            h,
            event_loop,
        }
    }

    fn show_image(
        vao: gl::types::GLuint,
        tex: &GlTexture,
        shader: &GlShader,
        receiver: &mut Receiver,
    ) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        //for (pos, rgb) in receiver.iter().take(200) {
            //tex.set_pixel((pos.x, pos.y), rgb);
        //}

        receiver.try_iter().for_each(|(pos, rgb)| {
            tex.set_pixel((pos.x, pos.y), rgb);
        });

        shader.bind();
        shader.uniform_texture("_Tex".to_string(), tex, 0);

        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }
}
