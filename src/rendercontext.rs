use std::usize;
use color::Color;

// The render context is the data structure
// that holds state about the current render.
// 
// It needs to be thread safe.
pub struct RenderContext {
    image: Vec<Color>,
    pub width: i32,
    pub height: i32
}

impl RenderContext {
    pub fn new(width:i32, height:i32) -> RenderContext {
        return RenderContext {
            image: vec![Color::black(); (width*height) as usize],
            width: width,
            height: height
        }
    }

    pub fn setPixel(&mut self, x: i32, y: i32, c:Color) {
        self.image[ (y*self.width + x) as usize ] = c;
    
    }
}
