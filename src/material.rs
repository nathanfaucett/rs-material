use alloc::boxed::Box;
use collections::string::String;

use core::any::Any;
use core::hash::{Hash, Hasher};

use uuid::Uuid;

use hash_map::HashMap;
use shared::Shared;
use gl_context::{CullFace, Blending};
use shader::Shader;


pub struct MaterialData {
    uuid: Uuid,

    shader: Option<Shader>,

    cull_face: CullFace,
    blending: Blending,

    wireframe: bool,
    wireframe_line_width: f32,

    uniforms: HashMap<String, Box<Any>>,
}

#[derive(Clone)]
pub struct Material {
    data: Shared<MaterialData>,
}

impl Material {

    pub fn new() -> Self {
        Material {
            data: Shared::new(MaterialData {
                uuid: Uuid::new_v4(),

                shader: None,

                cull_face: CullFace::Back,
                blending: Blending::Default,

                wireframe: false,
                wireframe_line_width: 1f32,

                uniforms: HashMap::new(),
            })
        }
    }

    pub fn get_uuid(&self) -> &Uuid {&self.data.uuid}

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

    pub fn get_uniforms(&self) -> &HashMap<String, Box<Any>> {
        &self.data.uniforms
    }
    pub fn get_uniforms_mut(&mut self) -> &mut HashMap<String, Box<Any>> {
        &mut self.data.uniforms
    }
}

impl Hash for Material {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
         (&*self.data as *const _).hash(state);
    }
}

impl PartialEq<Material> for Material {
    fn eq(&self, other: &Material) -> bool {
        let ref a = self.data;
        let ref b = other.data;
        
        let same_shader = if a.shader.is_some() && b.shader.is_some() {
            a.shader.as_ref().unwrap().eq(a.shader.as_ref().unwrap())
        } else {
            false
        };

        return
            same_shader &&

            a.cull_face == b.cull_face &&
            a.blending == b.blending &&

            a.wireframe == b.wireframe &&
            a.wireframe_line_width == b.wireframe_line_width
    }
}
impl Eq for Material {}
