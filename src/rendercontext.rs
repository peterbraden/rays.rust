use color::Color;
use std::u8;
// The render context is the data structure
// that holds state about the current render.
// 
// It needs to be thread safe.
pub struct RenderContext {
    image: Vec<Color>,
    pub width: u32,
    pub height: u32
}

impl RenderContext {
    pub fn new(width:u32, height:u32) -> RenderContext {
        return RenderContext {
            image: vec![Color::black(); (width*height) as usize],
            width: width,
            height: height
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, c:Color) {
        self.image[ (y*self.width + x) as usize ] = c;
    }

    pub fn get_pixel(&self, x:u32, y:u32) -> Color {
        return self.image[ (y*self.width + x) as usize ]
    }

    pub fn get_pixel_array(&self) -> Vec<u8> {
        let len = (self.width * self.height) as usize;
        let mut out: Vec<u8> = vec![0; len * 3];

        for i in  0 .. len {
            let (r, g, b) = self.image[i].to_u8();
            out.push(r);
            out.push(g);
            out.push(b);
            print!("{} {} {} {} {} \n", i, self.image[i], r, g, b);
        }

        return out;
    }
}
