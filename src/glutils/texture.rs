use std::os::raw::c_void;
use std::path::Path;
use gl::types::*;
use image::{self, GenericImage, DynamicImage::*};

#[derive(Clone, Default)]
pub struct Texture {
    id: GLuint,
    path: String,
    type_: String
}

impl Texture {
    pub fn new(path: &str, type_: Option<&str>) -> Self {
        let mut id: GLuint = 0;
        let img = image::open(&Path::new(path)).expect(&format!("Texture failed to load: {}", path));
        let format = match img {
            ImageLuma8(_) => gl::RED,
            ImageLumaA8(_) => gl::RG,
            ImageRgb8(_) => gl::RGB,
            ImageRgba8(_) => gl::RGBA
        };
        let data = img.raw_pixels();

        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                format,
                gl::UNSIGNED_BYTE,
                &data[0] as *const u8 as *const c_void
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            let wrap_mode = match type_ {
                Some("texture_normal") | Some("texture_specular") => gl::CLAMP_TO_EDGE as i32,
                _ => gl::REPEAT as i32,
            };

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap_mode);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap_mode);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }

        Self { 
            id,
            path: path.to_string(),
            type_: match type_ {
                Some(t) => t.to_string(),
                None => "texture_diffuse".to_string(),
            }
        }
    }

    pub fn active(&self, id: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + id);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn get_type(&self) -> String {
        self.type_.clone()
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
