use std::fs;

pub enum ShaderType {
    None,
    Fragment,
    Vertex,
}
pub struct Shader {
    pub source: String,
    pub shader_type: ShaderType,
}

pub struct ShaderData {
    pub source_path: String,
    pub vertex_shader: Shader,
    pub fragment_shader: Shader,
}

impl ShaderData {
    pub fn new(source_path: String) -> ShaderData {
        #[cfg(target_os = "macos")]
        let glsl_version = "#version 410";
        #[cfg(not(target_os = "macos"))]
        let glsl_version = "#version 130";

        let source =
            fs::read_to_string(source_path.to_string()).expect("File not found or missing");

        let mut vertex_shader = format!("{}\n", glsl_version).to_string();
        let mut fragment_shader = vertex_shader.clone();

        let mut shader_type = ShaderType::None;

        source.split("\n").for_each(|line| {
            if line.starts_with("#shader") {
                if line.contains("vertex") {
                    shader_type = ShaderType::Vertex;
                } else if line.contains("fragment") {
                    shader_type = ShaderType::Fragment;
                } else {
                    panic!("Unknown shader type");
                }
            } else {
                match shader_type {
                    ShaderType::None => (),
                    ShaderType::Vertex => {
                        vertex_shader += line;
                        vertex_shader += "\n";
                    }
                    ShaderType::Fragment => {
                        fragment_shader += line;
                        fragment_shader += "\n";
                    }
                }
            }
        });

        ShaderData {
            source_path,
            vertex_shader: Shader {
                source: vertex_shader,
                shader_type: ShaderType::Vertex,
            },
            fragment_shader: Shader {
                source: fragment_shader,
                shader_type: ShaderType::Fragment,
            },
        }
    }
}
