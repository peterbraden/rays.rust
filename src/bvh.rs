
pub struct BVH {}



impl struct BVH {}


impl Geometry for BVH {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection>;
    fn bounds(&self) -> BBox;
    fn primitives(&self) -> u64 { 1 }
}
