extern crate image;

use std::io::Write;
use crate::output::{LinearImage, OutputOpts};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

/// Preview the image in the terminal.
pub fn poor_mans(img: &LinearImage, opts: &OutputOpts) {
    let bufwtr = BufferWriter::stderr(ColorChoice::Auto);
    let mut buffer = bufwtr.buffer();
    let w = img.width.min(120);
    let h = ((img.height as f64 / img.width as f64) * w as f64 * 0.5) as usize; // * 0.5 as characters are roughly twice as tall as wide

    for y in 0 .. h {
        for x in 0 .. w {
            let c = img.get(((x as f32/ w as f32) * img.width as f32) as usize , ((y as f32 / h as f32) * img.height as f32) as usize);
            let [r, g, b] = opts.quantise(c);
            let termcol = Color::Rgb(r, g, b);
            buffer.set_color(ColorSpec::new().set_fg(Some(termcol))).expect("Could not set color");
            write!(&mut buffer, "█").expect("Could not write");
        }
        writeln!(&mut buffer).expect("Could not write");
    }

    buffer.set_color(&ColorSpec::new()).expect("Could not set color");
    write!(&mut buffer, "").expect("Could not write");
    bufwtr.print(&buffer).expect("Could not write");
}
