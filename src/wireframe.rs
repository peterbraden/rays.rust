use na::Vector3;
use scene::Scene;
use rendercontext::RenderContext;
use color::Color;


pub fn wireframe(s: &Scene, ctx: &mut RenderContext){
    // Draw wireframe of scene / bounding boxes

    for obj in s.objects.items() { 
        let b = obj.bounds();

        draw_point(b.min, Color::white(), s, ctx);
        draw_point(b.max, Color::white(), s, ctx);
    }
}



pub fn draw_point(pt:Vector3<f64>, c:Color, s: &Scene, rc: &mut RenderContext) {

    let (x, y) = s.camera.get_coord_for_point(pt);

    rc.set_pixel(
        (x * s.width as f64) as u32, 
        (y * s.height as f64) as u32,
        c);
}
