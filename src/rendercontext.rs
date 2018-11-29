extern crate time;

use scene::Scene;
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
    pub progressive_render: bool,
    pub pixels_rendered: u64,
}

fn format_f64(v: f64) -> String {
    if v > 1000000. {
        return format!("{:.2}M", v / 1000000.);
    }
    if v > 1000. {
        return format!("{:.2}K", v / 1000.);
    }
    return format!("{:.2}", v);
}

impl RenderContext {
    pub fn new(width:u32, height:u32, progressive_render: bool) -> RenderContext {
        return RenderContext {
            image: vec![Color::black(); (width*height) as usize],
            width: width,
            height: height,
            rays_cast: 0,
            start_time: time::precise_time_s(),
            progressive_render: progressive_render,
            pixels_rendered: 0,
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, c:Color) {
        self.image[ (y*self.width + x) as usize ] = c;
        self.pixels_rendered += 1;
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

    pub fn print_scene_stats(&self, s: &Scene){
    
        print!("# ============== Scene ===================\n");
        print!("| Objects: {}\n", s.objects);
        print!("| - Primitives: {}\n", s.objects
                                        .items
                                        .iter()
                                        .map(|x| x.geometry.primitives())
                                        .fold(0, |acc, x| acc + x));
        print!("# ========================================\n");
    }
    
    pub fn print_progress(&self, _x: u32, _y: u32){
        let elapsed = time::precise_time_s() - self.start_time;
        println!("- [{:.0}s] {} rays cast ({} RPS), {} Rays per pixel, {}%, {} threads",
                 elapsed,
                 format_f64(self.rays_cast as f64),
                 format_f64(self.rays_cast as f64 / elapsed),
                 format_f64(self.rays_cast as f64 / self.pixels_rendered as f64),
                 format_f64((self.pixels_rendered as f64 / (self.width * self.height) as f64) * 100.),
                 rayon::current_num_threads());
    }
}
