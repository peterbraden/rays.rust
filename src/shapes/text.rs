//! Text: a string's glyphs from a TrueType or OpenType font, extruded into solid letters (POV-Ray
//! 3.7 reference: "Text").
//!
//! As in POV-Ray, the letters stand in the xy plane on the baseline y = 0, starting at x = 0,
//! about one unit high (the font's em square is one unit), and extend from z = 0 back to
//! z = `thickness`. Each glyph's outline is flattened into straight edges and swept as a prism.
//! Characters are placed by their advance widths, without kerning.
//!
//! These are POV-Ray's coordinates, but POV-Ray is left-handed and this renderer right-handed,
//! so here the text reads left to right when seen from the +z side, looking towards -z (seen from
//! -z, as in POV-Ray, it is mirrored; turn it 180 degrees about y to face that way). A POV-Ray
//! scene converted by negating z throughout reads as it did there.

use crate::shapes::geometry::Geometry;
use crate::na::{Rotation3, Vector2, Vector3};
use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::bbox::BBox;
use crate::shapes::polygon::{Outline, Prism, Sweep};
use crate::shapes::transform::Transform;

type P2 = Vector2<f64>;

/// Collects a glyph's contours as loops of points, flattening curves.
struct Flattener {
    loops: Vec<Vec<P2>>,
    current: Vec<P2>,
    scale: f64,
    origin: P2,
    steps: usize,
}

impl Flattener {
    fn p(&self, x: f32, y: f32) -> P2 {
        self.origin + P2::new(x as f64, y as f64) * self.scale
    }

    fn last(&self) -> P2 {
        *self.current.last().unwrap_or(&self.origin)
    }
}

impl ttf_parser::OutlineBuilder for Flattener {
    fn move_to(&mut self, x: f32, y: f32) {
        if !self.current.is_empty() {
            self.loops.push(std::mem::take(&mut self.current));
        }
        self.current.push(self.p(x, y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.current.push(self.p(x, y));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let (a, c, b) = (self.last(), self.p(x1, y1), self.p(x, y));
        for i in 1..=self.steps {
            let t = i as f64 / self.steps as f64;
            let s = 1. - t;
            self.current.push(a * (s * s) + c * (2. * s * t) + b * (t * t));
        }
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let (a, c1, c2, b) = (self.last(), self.p(x1, y1), self.p(x2, y2), self.p(x, y));
        for i in 1..=self.steps {
            let t = i as f64 / self.steps as f64;
            let s = 1. - t;
            self.current.push(a * (s * s * s) + c1 * (3. * s * s * t) + c2 * (3. * s * t * t) + b * (t * t * t));
        }
    }

    fn close(&mut self) {
        if let (Some(first), Some(last)) = (self.current.first(), self.current.last())
            && first == last {
            self.current.pop();
        }
        if !self.current.is_empty() {
            self.loops.push(std::mem::take(&mut self.current));
        }
    }
}

/// The glyphs as prisms swept along y, outlines in the xz plane: turned upright by `text`.
struct Glyphs {
    prisms: Vec<(Prism, BBox)>,
    bounds: BBox,
}

impl Geometry for Glyphs {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        let rd = r.rd.normalize();
        let inv = Vector3::new(1. / rd.x, 1. / rd.y, 1. / rd.z);
        let ray = Ray { ro: r.ro, rd };
        self.prisms.iter()
            .filter(|(_, b)| b.entry_exit(&r.ro, &inv).is_some())
            .filter_map(|(p, _)| p.intersects(&ray))
            .min_by(|a, b| a.dist.total_cmp(&b.dist))
    }

    fn bounds(&self) -> BBox {
        self.bounds
    }

    fn primitives(&self) -> u64 {
        self.prisms.len() as u64
    }

    fn inside(&self, p: &Vector3<f64>) -> bool {
        self.prisms.iter().any(|(g, _)| g.inside(p))
    }
}

/// `string` set in the font at `font_path`, `thickness` deep, with `offset` added between
/// characters (beyond their advance widths). Curves are flattened into `steps` edges.
pub fn text(font_path: &str, string: &str, thickness: f64, offset: P2, steps: usize) -> Box<dyn Geometry + Sync + Send> {
    assert!(thickness > 0., "text thickness must be positive");
    let data = std::fs::read(font_path).unwrap_or_else(|e| panic!("Failed to read font {}: {}", font_path, e));
    let face = ttf_parser::Face::parse(&data, 0).unwrap_or_else(|e| panic!("Failed to parse font {}: {}", font_path, e));
    let scale = 1. / face.units_per_em() as f64;

    let mut prisms = Vec::new();
    let mut pen = P2::zeros();
    for ch in string.chars() {
        let Some(id) = face.glyph_index(ch) else {
            println!("Warning: font {} has no glyph for {:?}", font_path, ch);
            continue;
        };
        let mut f = Flattener { loops: vec![], current: vec![], scale, origin: pen, steps };
        face.outline_glyph(id, &mut f);
        ttf_parser::OutlineBuilder::close(&mut f);
        let loops: Vec<Vec<P2>> = f.loops.into_iter().filter(|l| l.len() >= 3).collect();
        if !loops.is_empty() {
            let prism = Prism::new(Outline::new(loops), 0., thickness, Sweep::Linear, false);
            let b = prism.bounds();
            prisms.push((prism, b));
        }
        pen += P2::new(face.glyph_hor_advance(id).unwrap_or(0) as f64 * scale, 0.) + offset;
    }
    assert!(!prisms.is_empty(), "text \"{}\" has no visible glyphs", string);
    let bounds = prisms.iter().fold(BBox::empty(), |b, (_, pb)| b.union(pb));

    // Prism space has the glyph's (x, y) as (x, z) and depth as y. Turning -90 degrees about x
    // takes (x, d, y) to (x, y, -d); moving back by the thickness puts depth at z = 0..thickness.
    Box::new(Transform::from_parts(
        std::sync::Arc::new(Glyphs { prisms, bounds }) as std::sync::Arc<dyn Geometry + Sync + Send>,
        Vector3::new(0., 0., thickness),
        Rotation3::from_axis_angle(&Vector3::x_axis(), -std::f64::consts::FRAC_PI_2),
        Vector3::new(1., 1., 1.)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const FONT: &str = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf";

    #[test]
    fn test_letters_stand_on_the_baseline() {
        if !std::path::Path::new(FONT).exists() {
            println!("skipping: no {}", FONT);
            return;
        }
        let t = text(FONT, "IO", 0.5, P2::zeros(), 8);
        let b = t.bounds();
        assert!(b.min.y.abs() < 0.05 && b.max.y > 0.6 && b.max.y < 1., "{}", b);
        assert!(b.min.z.abs() < 1e-9 && (b.max.z - 0.5).abs() < 1e-9, "{}", b);
        // The I's stem, from the front: the front face at z = 0 facing -z.
        let stem = b.min.x + 0.05;
        let i = t.intersects(&Ray { ro: Vector3::new(stem, 0.3, -5.), rd: Vector3::new(0., 0., 1.) }).unwrap();
        assert!((i.dist - 5.).abs() < 1e-9, "{}", i.dist);
        assert!((i.normal - Vector3::new(0., 0., -1.)).norm() < 1e-9, "{}", i.normal);
        // Through the hole of the O: find its middle from the bounds of the second glyph.
        let o_mid = b.max.x - 0.4;
        assert!(t.intersects(&Ray { ro: Vector3::new(o_mid, 0.36, -5.), rd: Vector3::new(0., 0., 1.) }).is_none());
        // Down onto the top of the I.
        let top = t.intersects(&Ray { ro: Vector3::new(stem, 5., 0.25), rd: Vector3::new(0., -1., 0.) }).unwrap();
        assert!((top.normal - Vector3::new(0., 1., 0.)).norm() < 1e-9);
    }
}
