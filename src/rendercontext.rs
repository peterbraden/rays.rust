use std::usize;

// The render context is the data structure
// that holds state about the current render.
// 
// It needs to be thread safe.
pub struct RenderContext {
    image: Vec<i32>
}

impl RenderContext {
    pub fn new(width:i32, height:i32) -> RenderContext {
        return RenderContext {
            image: vec![0; (width*height) as usize]
        }
    }
}
