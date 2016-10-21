extern crate time;

use color::Color;
// The render context is the data structure
// that holds state about the current render.
// 
// It needs to be thread safe.
pub struct RenderContext {
    image: Vec<Color>,
    pub width: u32,
    pub height: u32,
    pub rays_cast: u64,

    pub start_time: f64,
}

impl RenderContext {
    pub fn new(width:u32, height:u32) -> RenderContext {
        return RenderContext {
            image: vec![Color::black(); (width*height) as usize],
            width: width,
            height: height,
            rays_cast: 0,
            start_time: time::precise_time_s(),
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, c:Color) {
        if x >= self.width || y.saturating_mul(self.width).saturating_add(x) >= self.width * self.height {
            return;
        }

        self.image[ (y*self.width + x) as usize ] = c;
    }

    pub fn set_pixel_opacity(&mut self, x: u32, y: u32, opacity:f64, c:Color) {
        if x >= self.width || y.saturating_mul(self.width).saturating_add(x) >= self.width * self.height {
            return;
        }

        let prev = self.image[ (y*self.width + x) as usize ];
        self.image[ (y*self.width + x) as usize ] = (prev * (1.-opacity) + (c * opacity));
    }


    pub fn get_pixel(&self, x:u32, y:u32) -> Color {
        return self.image[ (y*self.width + x) as usize ]
    }
/*
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
    */
    pub fn print_stats(&self) {
        let elapsed = time::precise_time_s() - self.start_time;

        print!("\n==========================================\n");
        print!("| Rays Cast: {}\n", self.rays_cast);
        print!("| Elapsed Time (s): {:.4}\n", elapsed);
        print!("| Rays per sec: {:.2}\n", self.rays_cast as f64 / elapsed);
        print!("==========================================\n");

    }
}
