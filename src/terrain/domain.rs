//! # The simulation grid
//!
//! Erosion is simulated on a set of cells covering either a whole planet or a square patch of
//! one. Both are presented to the processes as the same thing: a graph of cells, each with an
//! area and a position on the planet, joined by links with a length and (for links between cells
//! that share an edge) a width. Flow routing and the river solver only need the graph (Braun and
//! Willett 2013 work on an arbitrary "stack" of nodes); ice flux is a finite-volume sum over the
//! edge-sharing links. So the processes are written once and run on either layout.
//!
//! Every cell is placed by its direction from the planet's centre (a unit vector), and the
//! planet's noise fields are functions of that direction. A patch and the whole planet with the
//! same seed therefore describe the same world: a patch is a higher-resolution window onto it.
//!
//! ## The whole planet: a cubed sphere
//!
//! A latitude-longitude grid crowds cells at the poles. Instead, project a cube's six faces onto
//! the sphere (Sadourny, "Conservative finite-difference approximations of the primitive
//! equations on quasi-uniform spherical grids", Monthly Weather Review 100(2), 1972,
//! doi:10.1175/1520-0493(1972)100<0136:CFAOTP>2.3.CO;2). Ronchi, Iacono and Paolucci ("The
//! 'Cubed Sphere'", J. Comput. Phys. 124, 1996, doi:10.1006/jcph.1996.0047) use the
//! equiangular gnomonic projection, as here: a face coordinate a in [-1, 1] maps to the angle
//! α = aπ/4, and the direction is normal + tan(α) u + tan(β) v, normalised. Equal steps in angle
//! make the cells far more uniform in size than equal steps on the cube's face (the largest cell
//! is about 1.3 times the smallest, rather than 5 times).
//!
//! Neighbours are found geometrically: step one cell in face coordinates, possibly off the edge
//! of the face, and ask which cell contains that direction (`cell_at`). This crosses face edges
//! without a table of how the faces join. At the cube's eight corners three faces meet, so those
//! cells have seven neighbours rather than eight.
//!
//! ## A patch
//!
//! A square of n x n cells of equal size on the plane tangent to the planet at its centre, each
//! cell's direction found by projecting from the planet's centre (gnomonic). For patches small
//! compared to the planet the cells are near-square on the sphere too. Cells on the patch's
//! edge are boundary cells: the processes hold them fixed, as base level, and water and ice
//! leave the patch through them.
//!
//! ## Corners, for rendering
//!
//! The renderer triangulates the cells' corners, each corner's height being the mean of the
//! cells that meet there. Corners on a face edge of the cubed sphere appear on both faces with
//! the same direction and the same cells, hence the same height, so the mesh has no cracks.

use crate::na::Vector3;
use std::f64::consts::FRAC_PI_4;

/// A link from one cell to a neighbour.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Link {
    pub to: usize,
    /// Distance between the cell centres, in metres.
    pub dist: f64,
    /// Length of the edge the two cells share, in metres; 0 for diagonal neighbours.
    pub width: f64,
}

#[derive(Clone, Debug)]
pub enum Layout {
    /// Cell (i, j) is at `up * radius + east * x + north * z` projected onto the sphere, with
    /// x = (i + 0.5 - n/2) * spacing, and z likewise for j.
    Patch { up: Vector3<f64>, east: Vector3<f64>, north: Vector3<f64>, n: usize, spacing: f64 },
    /// Cell (face, i, j) is at index face * n * n + j * n + i.
    Sphere { n: usize },
}

pub struct Domain {
    pub layout: Layout,
    /// Planet radius, metres.
    pub radius: f64,
    /// Each cell centre's direction from the planet's centre.
    pub dirs: Vec<Vector3<f64>>,
    /// Each cell's area, square metres.
    pub area: Vec<f64>,
    /// Cells on the edge of a patch (none on a sphere).
    pub boundary: Vec<bool>,
    link_start: Vec<usize>,
    links: Vec<Link>,
    /// Corner directions, for rendering.
    pub corners: Vec<Vector3<f64>>,
    /// The cells meeting at each corner, sorted.
    pub corner_cells: Vec<Vec<usize>>,
    /// Each cell's four corners, anticlockwise seen from outside the planet.
    pub quads: Vec<[usize; 4]>,
}

/// A face of the cube: its outward normal and the two axes along it, with u x v = normal.
const FACES: [([f64; 3], [f64; 3], [f64; 3]); 6] = [
    ([1., 0., 0.], [0., 1., 0.], [0., 0., 1.]),
    ([-1., 0., 0.], [0., 0., 1.], [0., 1., 0.]),
    ([0., 1., 0.], [0., 0., 1.], [1., 0., 0.]),
    ([0., -1., 0.], [1., 0., 0.], [0., 0., 1.]),
    ([0., 0., 1.], [1., 0., 0.], [0., 1., 0.]),
    ([0., 0., -1.], [0., 1., 0.], [1., 0., 0.]),
];

fn face_axes(f: usize) -> (Vector3<f64>, Vector3<f64>, Vector3<f64>) {
    let (n, u, v) = FACES[f];
    (Vector3::from(n), Vector3::from(u), Vector3::from(v))
}

/// The direction at face coordinates (a, b), each in [-1, 1] on the face (and beyond it, off the
/// edge, which is how neighbours across face edges are found).
fn sphere_dir(f: usize, a: f64, b: f64) -> Vector3<f64> {
    let (n, u, v) = face_axes(f);
    (n + u * (a * FRAC_PI_4).tan() + v * (b * FRAC_PI_4).tan()).normalize()
}

/// Solid angle of the spherical triangle with unit-vector corners a, b, c (Van Oosterom and
/// Strackee, "The Solid Angle of a Plane Triangle", IEEE Trans. Biomed. Eng., 1983).
fn solid_angle(a: &Vector3<f64>, b: &Vector3<f64>, c: &Vector3<f64>) -> f64 {
    let num = a.dot(&b.cross(c)).abs();
    let den = 1. + a.dot(b) + b.dot(c) + c.dot(a);
    2. * num.atan2(den)
}

impl Domain {
    pub fn len(&self) -> usize { self.dirs.len() }

    pub fn links(&self, i: usize) -> &[Link] {
        &self.links[self.link_start[i] .. self.link_start[i + 1]]
    }

    /// The local "up" direction at latitude and longitude (degrees), and the east and north
    /// directions there. The planet's axis is +y, and it spins anticlockwise about it (seen from
    /// above the north pole), so east x north = up.
    pub fn frame(lat: f64, lon: f64) -> (Vector3<f64>, Vector3<f64>, Vector3<f64>) {
        let (lat, lon) = (lat.to_radians(), lon.to_radians());
        let up = Vector3::new(lat.cos() * lon.cos(), lat.sin(), -lat.cos() * lon.sin());
        let east = Vector3::new(-lon.sin(), 0., -lon.cos());
        let north = up.cross(&east);
        (up, east, north)
    }

    /// An n x n patch of cells `spacing` metres across, centred at a latitude and longitude, on a
    /// planet of `radius` metres.
    pub fn patch(lat: f64, lon: f64, n: usize, spacing: f64, radius: f64) -> Domain {
        assert!(n >= 2);
        let (up, east, north) = Domain::frame(lat, lon);
        let half = n as f64 / 2.;
        let to_dir = |x: f64, z: f64| (up * radius + east * x + north * z).normalize();

        let mut dirs = Vec::with_capacity(n * n);
        let mut boundary = Vec::with_capacity(n * n);
        for j in 0..n {
            for i in 0..n {
                dirs.push(to_dir((i as f64 + 0.5 - half) * spacing, (j as f64 + 0.5 - half) * spacing));
                boundary.push(i == 0 || j == 0 || i == n - 1 || j == n - 1);
            }
        }

        let mut link_start = vec![0];
        let mut links = vec![];
        for j in 0..n as i64 {
            for i in 0..n as i64 {
                // Edge-sharing neighbours first, then diagonals.
                for (di, dj) in [(1, 0), (0, 1), (-1, 0), (0, -1), (1, 1), (-1, 1), (-1, -1), (1, -1)] {
                    let (ni, nj) = (i + di, j + dj);
                    if ni < 0 || nj < 0 || ni >= n as i64 || nj >= n as i64 { continue; }
                    let diagonal = di != 0 && dj != 0;
                    links.push(Link {
                        to: nj as usize * n + ni as usize,
                        dist: if diagonal { spacing * 2f64.sqrt() } else { spacing },
                        width: if diagonal { 0. } else { spacing },
                    });
                }
                link_start.push(links.len());
            }
        }

        let mut corners = Vec::with_capacity((n + 1) * (n + 1));
        let mut corner_cells = Vec::with_capacity((n + 1) * (n + 1));
        for cj in 0..=n {
            for ci in 0..=n {
                corners.push(to_dir((ci as f64 - half) * spacing, (cj as f64 - half) * spacing));
                let mut cells = vec![];
                for (i, j) in [(ci as i64 - 1, cj as i64 - 1), (ci as i64, cj as i64 - 1), (ci as i64 - 1, cj as i64), (ci as i64, cj as i64)] {
                    if i >= 0 && j >= 0 && i < n as i64 && j < n as i64 {
                        cells.push(j as usize * n + i as usize);
                    }
                }
                corner_cells.push(cells);
            }
        }
        let c = |ci: usize, cj: usize| cj * (n + 1) + ci;
        let quads = (0..n * n).map(|k| {
            let (i, j) = (k % n, k / n);
            // east x north = up, so (east, north) order is anticlockwise seen from above.
            [c(i, j), c(i + 1, j), c(i + 1, j + 1), c(i, j + 1)]
        }).collect();

        Domain {
            layout: Layout::Patch { up, east, north, n, spacing },
            radius,
            area: vec![spacing * spacing; n * n],
            dirs, boundary, link_start, links, corners, corner_cells, quads,
        }
    }

    /// A whole planet of `radius` metres, as a cubed sphere with n x n cells per face.
    pub fn sphere(n: usize, radius: f64) -> Domain {
        assert!(n >= 2);
        let step = 2. / n as f64;
        let coord = |i: usize| -1. + (i as f64 + 0.5) * step;
        let layout = Layout::Sphere { n };

        let mut dirs = Vec::with_capacity(6 * n * n);
        let mut area = Vec::with_capacity(6 * n * n);
        let mut quads = Vec::with_capacity(6 * n * n);
        let corner_index = |f: usize, ci: usize, cj: usize| f * (n + 1) * (n + 1) + cj * (n + 1) + ci;
        for f in 0..6 {
            for j in 0..n {
                for i in 0..n {
                    dirs.push(sphere_dir(f, coord(i), coord(j)));
                    let c = |ci: usize, cj: usize| sphere_dir(f, -1. + ci as f64 * step, -1. + cj as f64 * step);
                    let (c0, c1, c2, c3) = (c(i, j), c(i + 1, j), c(i + 1, j + 1), c(i, j + 1));
                    area.push(radius * radius * (solid_angle(&c0, &c1, &c2) + solid_angle(&c0, &c2, &c3)));
                    quads.push([corner_index(f, i, j), corner_index(f, i + 1, j), corner_index(f, i + 1, j + 1), corner_index(f, i, j + 1)]);
                }
            }
        }

        // Neighbours, made symmetric: near the cube's corners a diagonal step from one side
        // need not land where the reverse step does.
        let count = 6 * n * n;
        let mut nbrs: Vec<Vec<(usize, bool)>> = vec![vec![]; count];
        for f in 0..6 {
            for j in 0..n {
                for i in 0..n {
                    let k = f * n * n + j * n + i;
                    for (di, dj) in [(1, 0), (0, 1), (-1, 0), (0, -1), (1, 1), (-1, 1), (-1, -1), (1, -1)] {
                        let d = sphere_dir(f, coord(i) + di as f64 * step, coord(j) + dj as f64 * step);
                        let to = Domain::sphere_cell_at(n, &d);
                        if to == k { continue; }
                        let edge = di == 0 || dj == 0;
                        for (from, other) in [(k, to), (to, k)] {
                            match nbrs[from].iter_mut().find(|x| x.0 == other) {
                                Some(x) => x.1 |= edge,
                                None => nbrs[from].push((other, edge)),
                            }
                        }
                    }
                }
            }
        }
        let mut link_start = vec![0];
        let mut links = vec![];
        for k in 0..count {
            // Edge-sharing neighbours first, then by index, so the order doesn't depend on the
            // order the cells were visited in.
            nbrs[k].sort_by_key(|&(to, edge)| (!edge, to));
            for &(to, edge) in &nbrs[k] {
                let dist = radius * dirs[k].angle(&dirs[to]);
                let width = if edge { (area[k].sqrt() + area[to].sqrt()) / 2. } else { 0. };
                links.push(Link { to, dist, width });
            }
            link_start.push(links.len());
        }

        let mut corners = Vec::with_capacity(6 * (n + 1) * (n + 1));
        let mut corner_cells = Vec::with_capacity(6 * (n + 1) * (n + 1));
        let h = step / 2.;
        for f in 0..6 {
            for cj in 0..=n {
                for ci in 0..=n {
                    let (a, b) = (-1. + ci as f64 * step, -1. + cj as f64 * step);
                    corners.push(sphere_dir(f, a, b));
                    let mut cells: Vec<usize> = [(-h, -h), (h, -h), (-h, h), (h, h)].iter()
                        .map(|(da, db)| Domain::sphere_cell_at(n, &sphere_dir(f, a + da, b + db)))
                        .collect();
                    cells.sort();
                    cells.dedup();
                    corner_cells.push(cells);
                }
            }
        }

        Domain { layout, radius, dirs, area, boundary: vec![false; count], link_start, links, corners, corner_cells, quads }
    }

    fn sphere_cell_at(n: usize, d: &Vector3<f64>) -> usize {
        let ax = d.x.abs().max(d.y.abs()).max(d.z.abs());
        let f = if d.x.abs() == ax {
            if d.x > 0. { 0 } else { 1 }
        } else if d.y.abs() == ax {
            if d.y > 0. { 2 } else { 3 }
        } else if d.z > 0. { 4 } else { 5 };
        let (fnorm, u, v) = face_axes(f);
        let dn = d.dot(&fnorm);
        let index = |x: f64| {
            let a = (x / dn).atan() / FRAC_PI_4;
            (((a + 1.) / 2. * n as f64).floor() as i64).clamp(0, n as i64 - 1) as usize
        };
        f * n * n + index(d.dot(&v)) * n + index(d.dot(&u))
    }

    /// The cell containing direction `d` (which needn't be normalised). Directions off a patch
    /// give the nearest edge cell.
    pub fn cell_at(&self, d: &Vector3<f64>) -> usize {
        match &self.layout {
            Layout::Sphere { n } => Domain::sphere_cell_at(*n, d),
            Layout::Patch { up, east, north, n, spacing } => {
                let (x, z) = Domain::tangent(*up, *east, *north, self.radius, d);
                let index = |x: f64| ((x / spacing + *n as f64 / 2.).floor() as i64).clamp(0, *n as i64 - 1) as usize;
                index(z) * n + index(x)
            }
        }
    }

    /// Gnomonic tangent-plane coordinates (east, north), in metres, of direction `d`.
    pub fn tangent(up: Vector3<f64>, east: Vector3<f64>, north: Vector3<f64>, radius: f64, d: &Vector3<f64>) -> (f64, f64) {
        let p = d * (radius / d.dot(&up));
        (p.dot(&east), p.dot(&north))
    }

    /// The smallest distance between linked cells: the grid's resolution.
    pub fn min_spacing(&self) -> f64 {
        self.links.iter().filter(|l| l.width > 0.).map(|l| l.dist).fold(f64::INFINITY, f64::min)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    fn assert_symmetric(d: &Domain) {
        for i in 0..d.len() {
            for l in d.links(i) {
                let back = d.links(l.to).iter().find(|b| b.to == i).expect("link not symmetric");
                assert!((back.dist - l.dist).abs() < 1e-6 * l.dist);
                assert!((back.width - l.width).abs() < 1e-6 * l.dist);
            }
        }
    }

    #[test]
    fn test_sphere_area_is_whole_planet() {
        let d = Domain::sphere(8, 2.);
        let total: f64 = d.area.iter().sum();
        assert!((total - 4. * PI * 4.).abs() < 1e-9, "{}", total);
    }

    #[test]
    fn test_sphere_cells_are_roughly_uniform() {
        let d = Domain::sphere(16, 1.);
        let max = d.area.iter().cloned().fold(0., f64::max);
        let min = d.area.iter().cloned().fold(f64::INFINITY, f64::min);
        assert!(max / min < 1.5, "{}", max / min);
    }

    #[test]
    fn test_sphere_neighbours() {
        let d = Domain::sphere(6, 1.);
        assert_symmetric(&d);
        let mut counts = vec![0; 9];
        for i in 0..d.len() {
            let edges = d.links(i).iter().filter(|l| l.width > 0.).count();
            assert_eq!(edges, 4, "cell {} has {} edge neighbours", i, edges);
            counts[d.links(i).len()] += 1;
        }
        // Three cells meet at each of the cube's 8 corners.
        assert_eq!(counts[7], 24);
        assert_eq!(counts[8], d.len() - 24);
    }

    #[test]
    fn test_sphere_cell_at_finds_each_cell() {
        let d = Domain::sphere(5, 1.);
        for (i, dir) in d.dirs.iter().enumerate() {
            assert_eq!(d.cell_at(dir), i);
        }
    }

    #[test]
    fn test_corners_are_shared_across_face_edges() {
        let d = Domain::sphere(4, 1.);
        for (c, cells) in d.corner_cells.iter().enumerate() {
            assert!(cells.len() == 3 || cells.len() == 4, "{:?}", cells);
            for other in 0..d.corners.len() {
                if (d.corners[other] - d.corners[c]).norm() < 1e-9 {
                    assert_eq!(&d.corner_cells[other], cells);
                }
            }
        }
    }

    #[test]
    fn test_patch() {
        let d = Domain::patch(45., 7., 4, 100., 6.371e6);
        assert_symmetric(&d);
        assert_eq!(d.len(), 16);
        assert_eq!(d.boundary.iter().filter(|&&b| b).count(), 12);
        assert_eq!(d.links(5).len(), 8);
        assert_eq!(d.links(0).len(), 3);
        for (i, dir) in d.dirs.iter().enumerate() {
            assert_eq!(d.cell_at(dir), i);
        }
        assert_eq!(d.min_spacing(), 100.);
        // Neighbouring cells are 100 m apart on the sphere, too.
        let arc = d.radius * d.dirs[5].angle(&d.dirs[6]);
        assert!((arc - 100.).abs() < 0.01, "{}", arc);
    }

    #[test]
    fn test_frame() {
        let (up, east, north) = Domain::frame(30., 45.);
        assert!((east.cross(&north) - up).norm() < 1e-12);
        assert!((up.y - 0.5).abs() < 1e-12);
        // Heading east from (lat, lon) increases longitude.
        let (up2, _, _) = Domain::frame(30., 45.001);
        assert!((up2 - up).dot(&east) > 0.);
    }

    #[test]
    fn test_patch_quads_face_outwards() {
        let d = Domain::patch(-30., 100., 3, 1000., 6.371e6);
        for q in &d.quads {
            let (a, b, c) = (d.corners[q[0]], d.corners[q[1]], d.corners[q[2]]);
            assert!((b - a).cross(&(c - a)).dot(&a) > 0.);
        }
        let s = Domain::sphere(3, 1.);
        for q in &s.quads {
            let (a, b, c) = (s.corners[q[0]], s.corners[q[1]], s.corners[q[2]]);
            assert!((b - a).cross(&(c - a)).dot(&a) > 0.);
        }
    }
}
