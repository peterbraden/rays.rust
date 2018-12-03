use na::Vector3;
use scene::Scene;
use rendercontext::RenderContext;
use color::Color;
use std::mem;
use shapes::bbox::BBox;


pub fn wireframe(s: &Scene, ctx: &mut RenderContext){
    // Draw wireframe of scene / bounding boxes

    for obj in s.objects.items() { 
        let b = obj.geometry.bounds();
        draw_bbox(b, Color::red(), s, ctx);
    }

    //for b in s.objects.partitions() {
    //    draw_bbox(b, Color::blue(), s, ctx)
    //}
    //
    draw_line(Vector3::new(0., 0., 0.), Vector3::new(10., 0., 0.), Color::blue(), s, ctx);
    draw_line(Vector3::new(0., 0., 0.), Vector3::new(-10., 0., 0.), Color::blue(), s, ctx);

}

pub fn draw_bbox(b:BBox, c:Color,  s: &Scene, rc:&mut RenderContext) {
    let trf = b.max;
    let blc = b.min;
    draw_line(blc, Vector3::new(blc.x, blc.y, trf.z) , c, s, rc);
    draw_line(blc, Vector3::new(blc.x, trf.y, blc.z) , c, s, rc);
    draw_line(blc, Vector3::new(trf.x, blc.y, blc.z) , c, s, rc);

    draw_line(Vector3::new(trf.x, trf.y, blc.z), trf , c, s, rc);
    draw_line(Vector3::new(trf.x, blc.y, trf.z), trf , c, s, rc);
    draw_line(Vector3::new(blc.x, trf.y, trf.z), trf , c, s, rc);

    draw_line(Vector3::new(blc.x, trf.y, blc.z), Vector3::new(blc.x, trf.y, trf.z), c, s, rc);
    draw_line(Vector3::new(blc.x, trf.y, blc.z), Vector3::new(trf.x, trf.y, blc.z), c, s, rc);

    draw_line(Vector3::new(blc.x, blc.y, trf.z), Vector3::new(trf.x, blc.y, trf.z), c, s, rc);
    draw_line(Vector3::new(trf.x, blc.y, blc.z), Vector3::new(trf.x, blc.y, trf.z), c, s, rc);

    draw_line(Vector3::new(blc.x, blc.y, trf.z), Vector3::new(blc.x, trf.y, trf.z), c, s, rc);
    draw_line(Vector3::new(trf.x, blc.y, blc.z), Vector3::new(trf.x, trf.y, blc.z), c, s, rc);
}

pub fn draw_line(pt0: Vector3<f64>, pt1: Vector3<f64>, c:Color, s: &Scene, rc: &mut RenderContext) {
    // Bresenham’s Algorithm
    let mut steep = false; 

    let (x0s, y0s) = s.camera.get_coord_for_point(pt0);
    let (x1s, y1s) = s.camera.get_coord_for_point(pt1);
    let mut x0 = (x0s * s.width as f64) as i32;
    let mut y0 = (y0s * s.height as f64) as i32; 
    let mut x1 = (x1s * s.width as f64) as i32;
    let mut y1 = (y1s * s.height as f64) as i32;

    if (x0 - x1).abs() < (y0 - y1).abs() { 
        mem::swap(&mut x0, &mut y0); 
        mem::swap(&mut x1, &mut y1); 
        steep = true; 
    } 
    if x0 > x1 { 
        mem::swap(&mut x0, &mut x1); 
        mem::swap(&mut y0, &mut y1); 
    } 

    let dx = x1 - x0; 
    let dy = y1 - y0; 
    let derror = (dy as f64 /dx as f64).abs(); 
    let mut error = 0.; 
    let mut y = y0; 
    for x in x0..x1 { 
        if steep { 
            rc.set_pixel(y as u32, x as u32,c);
        } else { 
            rc.set_pixel(x as u32, y as u32,c);
        } 

        error += derror; 
        if error > 0.5 { 
            if y1 > y0 {
                y = y + 1
            } else {
                y = y - 1
            }
            error = error - 1.; 
        } 
    } 

}

pub fn draw_point(pt:Vector3<f64>, c:Color, s: &Scene, rc: &mut RenderContext) {
    let (x, y) = s.camera.get_coord_for_point(pt);
    draw_coord(x,y,c,s,rc);
}

pub fn draw_coord(x:f64, y:f64, c:Color, s: &Scene, rc: &mut RenderContext){
    rc.set_pixel(
        (x * s.width as f64) as u32, 
        (y * s.height as f64) as u32,
        c);
} 
