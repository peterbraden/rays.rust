extern crate image;

use std::io::Write;
use rendercontext::RenderContext;
use std::path::Path;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

pub fn to_png(ctx: &RenderContext) {
    let img = image::ImageBuffer::from_fn(ctx.width as u32, ctx.height as u32, |x, y| {
        let (r,g,b) = ctx.get_pixel(x as usize, ctx.height - y as usize - 1).to_u8();
        image::Rgb([r,g,b])
    });

    let _ = img.save(&Path::new(&ctx.output_filename));
}


pub fn poor_mans(ctx: &RenderContext) {
    let mut bufwtr = BufferWriter::stderr(ColorChoice::Auto);
    let mut buffer = bufwtr.buffer();
    let w = ctx.width.min(120);
    let h = ((ctx.height as f64 / ctx.width as f64) * w as f64 * 0.5) as usize; // * 0.5 as characters are roughly twice as tall as wide

    for y in 0 .. h {
        for x in 0 .. w {
            let c = ctx.get_pixel(((x as f32/ w as f32) * ctx.width as f32) as usize , (((h - 1 - y) as f32 / h as f32) * ctx.height as f32) as usize);
            let termcol = Color::Rgb((c.rgb[0] * 255.).round() as u8, (c.rgb[1] * 255.).round() as u8, (c.rgb[2] * 255.).round() as u8) ;
            buffer.set_color(ColorSpec::new().set_fg(Some(termcol)));
            write!(&mut buffer, "█");
        }
        write!(&mut buffer, "\n");
    }

    buffer.set_color(&ColorSpec::new());
    write!(&mut buffer, "");
    bufwtr.print(&buffer).expect("Could not write");
}
