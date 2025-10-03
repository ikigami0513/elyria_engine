use std::ffi::{CString, CStr};
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;

use gl;
use gl::types::*;

use cgmath::{Matrix, Matrix4, Vector3};
use cgmath::prelude::*;

use crate::core::path::get_path_to_asset;

pub struct Shader {
    pub id: u32
}

#[allow(dead_code)]
impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Shader {
        let mut shader = Shader { id: 0 };

        // 1. retrieve the vertex/fragment source code from filesystem
        let mut v_shader_file = File::open(get_path_to_asset(vertex_path))
            .unwrap_or_else(|_| panic!("Failed to open {}", vertex_path));

        let mut f_shader_file = File::open(get_path_to_asset(fragment_path))
            .unwrap_or_else(|_| panic!("Failed to open {}", fragment_path));

        let mut vertex_code = String::new();
        let mut fragment_code = String::new();

        v_shader_file
            .read_to_string(&mut vertex_code)
            .expect("Failed to read vertex shader");

        f_shader_file
            .read_to_string(&mut fragment_code)
            .expect("Failed to read fragment shader");

        let v_shader_code = CString::new(vertex_code.as_bytes()).unwrap();
        let f_shader_code = CString::new(fragment_code.as_bytes()).unwrap();

        // 2. compile shaders
        unsafe {
            // vertex shader
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &v_shader_code.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            shader.check_compile_errors(vertex, "VERTEX");

            // fragment shader
            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &f_shader_code.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            shader.check_compile_errors(fragment, "FRAGMENT");

            // shader program
            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);
            gl::LinkProgram(id);

            // delete the shaders as they're linked into our program now and no longer necessary
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            shader.id = id;
        }

        shader
    }

    // activate the shader
    pub unsafe fn use_program(&self) {
        unsafe { gl::UseProgram(self.id); }
    }

    pub unsafe fn set_bool(&self, name: &CStr, value: bool) {
        unsafe { gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value as i32); }
    }

    pub unsafe fn set_int(&self, name: &CStr, value: i32) {
        unsafe { gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value); }
    }

    pub unsafe fn set_float(&self, name: &CStr, value: f32) {
        unsafe { gl::Uniform1f(gl::GetUniformLocation(self.id, name.as_ptr()), value); }
    }

    pub unsafe fn set_vec3(&self, name: &CStr, value: &Vector3<f32>) {
        unsafe { gl::Uniform3fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, value.as_ptr()); }
    }

    pub unsafe fn set_mat4(&self, name: &CStr, mat: &Matrix4<f32>) {
        unsafe { gl::UniformMatrix4fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, gl::FALSE, mat.as_ptr()); }
    }

    unsafe fn check_compile_errors(&self, shader: u32, type_: &str) {
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(1024);
        unsafe { info_log.set_len(1024 - 1) }; // subtract 1 to skip the trailing null character
        if type_ != "PROGRAM" {
            unsafe { gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success) };
            if success != gl::TRUE as GLint {
                unsafe { gl::GetShaderInfoLog(shader, 1024, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar) };
                println!("ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                         type_,
                         str::from_utf8(&info_log).unwrap());
            }

        } else {
            unsafe { gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success) };
            if success != gl::TRUE as GLint {
                unsafe { gl::GetProgramInfoLog(shader, 1024, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar) };
                println!("ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                         type_,
                         str::from_utf8(&info_log).unwrap());
            }
        }
    }
}