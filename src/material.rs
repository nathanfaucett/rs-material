use alloc::boxed::Box;
use collections::btree_map::BTreeMap;
use collections::string::String;

use core::any::Any;

use shared::Shared;
use gl_context::{CullFace, Blending};
use shader::Shader;


pub struct MaterialData {
    shader: Option<Shader>,

    cull_face: CullFace,
    blending: Blending,

    wireframe: bool,
    wireframe_line_width: f32,

    receive_shadow: bool,
    cast_shadow: bool,

    uniforms: BTreeMap<String, Box<Any>>,
}

#[derive(Clone)]
pub struct Material {
    data: Shared<MaterialData>,
}

impl Material {

    pub fn new() -> Self {
        Material {
            data: Shared::new(MaterialData {
                shader: None,

                cull_face: CullFace::Back,
                blending: Blending::Default,

                wireframe: false,
                wireframe_line_width: 1f32,

                receive_shadow: true,
                cast_shadow: true,

                uniforms: BTreeMap::new(),
            })
        }
    }

    pub fn get_shader(&self) -> Option<Shader> {
        match self.data.shader {
            Some(ref shader) => Some(shader.clone()),
            None => None,
        }
    }
    pub fn set_shader(&mut self, shader: Shader) -> &mut Self {
        self.data.shader = Some(shader);
        self
    }

    pub fn get_cull_face(&self) -> CullFace {
        self.data.cull_face
    }
    pub fn set_cull_face(&mut self, cull_face: CullFace) -> &mut Self {
        self.data.cull_face = cull_face;
        self
    }

    pub fn get_blending(&self) -> Blending {
        self.data.blending
    }
    pub fn set_blending(&mut self, blending: Blending) -> &mut Self {
        self.data.blending = blending;
        self
    }

    pub fn get_wireframe(&self) -> bool {
        self.data.wireframe
    }
    pub fn set_wireframe(&mut self, wireframe: bool) -> &mut Self {
        self.data.wireframe = wireframe;
        self
    }

    pub fn get_wireframe_line_width(&self) -> f32 {
        self.data.wireframe_line_width
    }
    pub fn set_wireframe_line_width(&mut self, wireframe_line_width: f32) -> &mut Self {
        self.data.wireframe_line_width = wireframe_line_width;
        self
    }

    pub fn get_receive_shadow(&self) -> bool {
        self.data.receive_shadow
    }
    pub fn set_receive_shadow(&mut self, receive_shadow: bool) -> &mut Self {
        self.data.receive_shadow = receive_shadow;
        self
    }

    pub fn get_cast_shadow(&self) -> bool {
        self.data.cast_shadow
    }
    pub fn set_cast_shadow(&mut self, cast_shadow: bool) -> &mut Self {
        self.data.cast_shadow = cast_shadow;
        self
    }

    pub fn get_uniforms(&self) -> &BTreeMap<String, Box<Any>> {
        &self.data.uniforms
    }
    pub fn get_mut_uniforms(&mut self) -> &mut BTreeMap<String, Box<Any>> {
        &mut self.data.uniforms
    }
}
