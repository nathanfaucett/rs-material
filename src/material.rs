use collections::btree_map::BTreeMap;
use collections::string::String;

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

    uniforms: BTreeMap<String, usize>,
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
}
