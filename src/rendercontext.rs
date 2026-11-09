use std::time::{Instant};

use crate::scene::Scene;
use crate::color::Color;
use crate::trace::trace;
use crate::geometry::rand;


// The render context is the data structure
// that holds state about the current render.
// 
// It needs to be thread safe.
pub struct RenderContext {
    image: Vec<Color>,
    samples: Vec<usize>,
    pub width: usize,
    pub height: usize,
    pub rays_cast: u64,
    pub start_time: Instant,
    pub progressive_render: bool,
    pub samples_rendered: u64,
    pub output_filename: String,
}

pub struct RenderIterator {
    i: usize,
    pub width: usize,
    pub height: usize,
    pub samples: usize,
    pub chunk_size: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RenderableChunk {
    pub xmin: usize,
    pub xmax: usize,
    pub ymin: usize,
    pub ymax: usize,
    pub supersamples: usize,
}

pub struct RenderedChunk {
    pixels: Vec<Color>,
    samples: Vec<usize>,
    rays_cast: u64,
}


fn format_f64(v: f64) -> String {
    if v > 1000000. {
        format!("{:.2}M", v / 1000000.)
    } else if v > 1000. {
        format!("{:.2}K", v / 1000.)
    } else {
        format!("{:.2}", v)
    }
}

impl RenderContext {
    pub fn new(width:usize, height:usize, progressive_render: bool, filename: &str) -> RenderContext {
        let start_time = Instant::now();
        let output_filename = String::from(filename).replace(".json", ".png");
        RenderContext {
            image: vec![Color::black(); width*height],
            samples: vec![0; width*height],
            width,
            height,
            rays_cast: 0,
            start_time,
            progressive_render,
            samples_rendered: 0,
            output_filename,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, c:Color, samples: usize) {
        if x >= self.width || y.saturating_mul(self.width).saturating_add(x) >= self.width * self.height {
            return;
        }

        let i:usize = y*self.width + x;
        self.image[i] = self.image[i] + c.ignore_nan();
        self.samples[i] += samples;
        self.samples_rendered += samples as u64;
    }

    pub fn apply_chunk(&mut self, c: &RenderableChunk, p: &RenderedChunk){
        let mut i = 0;
        for y in c.ymin .. c.ymax {
            for x in c.xmin .. c.xmax {
                self.set_pixel(x, y, p.pixels[i], p.samples[i]);
                i += 1;
            }
        }
        self.rays_cast += p.rays_cast;
    }

    pub fn get_pixel(&self, x:usize, y:usize) -> Color {
        let i = y*self.width + x; 
        self.image[i] / self.samples[i].max(1) as f64
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
        let elapsed = Instant::now() - self.start_time;

        print!("\n==========================================\n");
        println!("| Rays Cast: {}", self.rays_cast);
        println!("| Elapsed Time: {:?}", elapsed);
        println!("| Rays per sec: {:.2}", self.rays_cast as f64 / elapsed.as_secs_f64());
        println!("==========================================");

    }

    
    pub fn progress(&self, s: &Scene) -> String {
        let elapsed = Instant::now() - self.start_time;
        format!("{:.2}s {} rays cast ({} RPS), {} Rays per sample, {}%, {} threads",
                 elapsed.as_secs_f64(),
                 format_f64(self.rays_cast as f64),
                 format_f64(self.rays_cast as f64 / elapsed.as_secs_f64()),
                 format_f64(self.rays_cast as f64 / self.samples_rendered as f64),
                 format_f64(self.progress_percentage(s)),
                 rayon::current_num_threads())
    }

    pub fn progress_percentage(&self, s: &Scene) -> f64 {
        (self.samples_rendered as f64 / (self.width * self.height * s.render.samples_per_pixel()) as f64) * 100.
    }

    pub fn iter(&self, s: &Scene) -> impl Iterator<Item=RenderableChunk> + '_ {
        let width = self.width;
        let height = self.height;
        let chunk_size = s.render.chunk_size;
        let chunk_layers = s.render.layers();
        let samples = s.render.samples_per_chunk.max(1);
        (0 .. chunk_layers)
                    .flat_map(move |_x| 
                        RenderIterator {
                            i: 0,
                            width,
                            height,
                            chunk_size,
                            samples,
                        })
    }
}

impl RenderableChunk {
    pub fn width(&self) -> usize {
        self.xmax - self.xmin
    }

    pub fn render(&self, s: &Scene) -> RenderedChunk {
        let size = self.width() * (self.ymax - self.ymin);
        let mut pixels: Vec<Color> = Vec::with_capacity(size);
        let mut samples: Vec<usize> = Vec::with_capacity(size);
        let mut rays_cast = 0;
        for y in self.ymin .. self.ymax {
            for x in self.xmin .. self.xmax {
                let (cast, psamples, pixel) = render_pixel(x, y, self.supersamples, s);
                pixels.push(pixel);
                samples.push(psamples);
                rays_cast += cast;
            }
        }

        RenderedChunk {
            pixels, samples, rays_cast
        }
    }   
}

impl Iterator for RenderIterator {
    type Item = RenderableChunk;

    fn next(&mut self) -> Option<RenderableChunk> {
        if self.i >= self.width * self.height {
            return None
        }

        // From i (pixel index) find current chunk
        let y = self.i / self.width;
        let x = self.i % self.width;

        if self.height - y > self.chunk_size {
            if self.width - x > self.chunk_size {
                self.i += self.chunk_size;
                Some(RenderableChunk {
                    xmin: x, 
                    xmax: x + self.chunk_size,
                    ymin: y,
                    ymax: y + self.chunk_size,
                    supersamples: self.samples,
                })
            } else {
                // Increment down a row
                self.i = (self.i - x) + (self.width * self.chunk_size);
                // return remainder of x
                Some(RenderableChunk {
                    xmin: x ,
                    xmax: self.width,
                    ymin: y,
                    ymax: y + self.chunk_size,
                    supersamples: self.samples,
                })
            }
        } else if self.width - x > self.chunk_size {
            self.i += self.chunk_size;
            Some(RenderableChunk {
                xmin: x, 
                xmax: x + self.chunk_size,
                ymin: y,
                ymax: self.height,
                supersamples: self.samples,
            })
        } else {
            self.i = (self.i - x) + self.chunk_size * self.width;
            Some(RenderableChunk {
                xmin: x ,
                xmax: self.width,
                ymin: y,
                ymax: self.height,
                supersamples: self.samples,
            })
        }
    }
}

fn render_pixel(x: usize, y: usize, max_samples: usize, s: &Scene) -> (u64, usize, Color) {
    let mut pixel = Color::black();
    let mut cast = 0;
    let mut samples = 0;

    // Monte-Carlo method: We sample many times and average.
    for sx in 0..max_samples {
        for sy in 0..max_samples {
            let (rays_cast, c) = trace(
                    &s.camera.get_ray(
                        x as f64 / (s.image.width as f64),
                        y as f64 / (s.image.height as f64),
                        // Stratified jitter within the pixel, so repeated layers sample
                        // new positions and edges are anti-aliased.
                        (sx as f64 + rand()) / (max_samples as f64) * 1. / (s.image.width as f64),
                        (sy as f64 + rand()) / (max_samples as f64) * 1. / (s.image.height as f64))
                    , 0, s);
            cast += rays_cast;
            pixel = pixel + c;
            samples += 1;
        }
    }
    (cast, samples, pixel)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenefile::SceneFile;

    fn scene(supersamples: usize, samples_per_chunk: usize) -> Scene {
        SceneFile::from_string(format!(r#"{{
            "width": 5, "height": 3, "chunk_size": 2,
            "supersamples": {}, "samples_per_chunk": {},
            "shadow_bias": 0.0001, "background": [0, 0, 0], "max_depth": 1,
            "camera": {{ "location": [0, 1, -1], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 }},
            "materials": {{}}, "media": {{}}, "lights": [], "objects": [], "variables": {{}}
        }}"#, supersamples, samples_per_chunk))
    }

    fn render(s: &Scene) -> RenderContext {
        let mut rc = RenderContext::new(s.image.width, s.image.height, false, "test.json");
        let chunks: Vec<RenderableChunk> = rc.iter(s).collect();
        for c in chunks {
            let p = c.render(s);
            rc.apply_chunk(&c, &p);
        }
        rc
    }

    #[test]
    fn test_samples_per_pixel_meets_supersamples() {
        for (supersamples, per_chunk) in [(35, 2), (1, 2), (8, 1), (9, 3), (4, 0)] {
            let s = scene(supersamples, per_chunk);
            let rc = render(&s);
            let expected = s.render.samples_per_pixel();
            assert!(expected >= supersamples, "{} < {}", expected, supersamples);
            for i in 0 .. s.image.width * s.image.height {
                assert_eq!(rc.samples[i], expected, "pixel {} for {:?}", i, (supersamples, per_chunk));
            }
            assert!((rc.progress_percentage(&s) - 100.).abs() < 1e-9);
        }
    }
}
