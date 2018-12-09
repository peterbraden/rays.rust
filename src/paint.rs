extern crate image;

use rendercontext::RenderContext;
use std::path::Path;

pub fn to_png(ctx: &RenderContext) {
    let img = image::ImageBuffer::from_fn(ctx.width as u32, ctx.height as u32, |x, y| {
        let (r,g,b) = ctx.get_pixel(x as usize, ctx.height - y as usize - 1).to_u8();
        image::Rgb([r,g,b])
    });

    let _ = img.save(&Path::new("demo/out.png"));
}


pub fn poor_mans(ctx: &RenderContext) {
    let w = 80;
    let h = 30;
    for y in 0 .. h {
        for x in 0 .. w {
            let c = ctx.get_pixel(((x as f32/ w as f32) * ctx.width as f32) as usize , (((h - 1 - y) as f32 / h as f32) * ctx.height as f32) as usize);
            let cx = (c.rgb[0] + c.rgb[1] + c.rgb[2]) / 3.;
            if cx < 0.05 {
                print!(" ");
            } else if cx < 0.1 {
                print!("▁");
            } else if cx < 0.2 {
                print!("▂");
            } else if cx < 0.3 {
                print!("▃");
            } else if cx < 0.4 {
                print!("▄");
            } else if cx < 0.5 {
                print!("▅");
            } else {
                print!("█");
            }
        }
        print!("\n");
    }
}
