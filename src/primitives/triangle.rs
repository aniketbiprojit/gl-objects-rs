use crate::object::OpenGLObject;

#[derive(Debug)]
pub(crate) struct Triangle {
    width: u32,
    height: u32,
    positions: [f32; 8],
}

impl OpenGLObject for Triangle {}
