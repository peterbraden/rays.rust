extern crate image;

use rendercontext::RenderContext;
use std::path::Path;

pub fn to_png(ctx: &RenderContext) {
    let img = image::ImageBuffer::from_fn(ctx.width, ctx.height, |x, y| {
        let (r,g,b) = ctx.get_pixel(x, y).to_u8();
        image::Rgb([r,g,b])
    });

    let _ = img.save(&Path::new("demo/out.png"));
}
