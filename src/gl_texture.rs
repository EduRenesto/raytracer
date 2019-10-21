type Rgb = vek::vec::repr_c::Rgb<f32>;

pub struct GlTexture {
    handle: gl::types::GLuint,
    width: gl::types::GLint,
    height: gl::types::GLint
}

impl GlTexture {
    pub fn new(width: usize, height: usize, data: Option<Vec<Rgb>>) -> GlTexture {
        let mut handle = 0 as gl::types::GLuint;

        let width = width as gl::types::GLint;
        let height = height as gl::types::GLint;

        unsafe {
            gl::GenTextures(1, &mut handle);
            gl::BindTexture(gl::TEXTURE_2D, handle);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            let data = match data {
                Some(data) => data.as_ptr() as *const std::ffi::c_void,
                None => std::ptr::null()
            };

            gl::TexImage2D(gl::TEXTURE_2D, 
                           0, gl::RGB as i32, width, height, 0, 
                           gl::RGB, gl::FLOAT, data);

            gl::ClearTexImage(handle, 0, gl::RGB, gl::FLOAT, std::ptr::null());
        }

        GlTexture { handle, width, height }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.handle);
        }
    }

    pub fn set_pixel(&self, pos: (usize, usize), rgb: Rgb) {
        let (u, v) = pos;

        let u = u as gl::types::GLint;
        let v = v as gl::types::GLint;

        self.bind();

        let data = rgb.into_array();
        unsafe {
            gl::TexSubImage2D(gl::TEXTURE_2D, 0, u, v, 1, 1, 
                              gl::RGB, gl::FLOAT, data.as_ptr() as *const std::ffi::c_void);
        }
    }
}
