//! Univariate polynomials and their real roots, for shapes whose intersection with a ray is the
//! root of a polynomial in the distance along it: the torus and general algebraic surfaces
//! (quartics), the lathe and surface of revolution (splines swept round an axis), and blobs (sums
//! of quartic field functions).
//!
//! Coefficients are stored lowest degree first: `[c0, c1, c2]` is c0 + c1 t + c2 t^2.
//!
//! Roots are isolated by the roots of the derivative: between consecutive critical points the
//! polynomial is monotonic, so it has at most one root there, found by bracketing. The critical
//! points are themselves the roots of a polynomial of one lower degree, found the same way. This
//! is the approach of Collins and Loos ("Real zeros of polynomials", in Buchberger, Collins and
//! Loos, "Computer Algebra: Symbolic and Algebraic Computation", 1983); POV-Ray uses Sturm
//! sequences for the same purpose (source/core/math/polynomialsolver.cpp). A root at which the
//! polynomial touches zero without crossing it (a ray grazing a surface) is missed, which is the
//! usual and harmless compromise.

/// Evaluate by Horner's rule.
pub fn eval(c: &[f64], t: f64) -> f64 {
    c.iter().rev().fold(0., |acc, &k| acc * t + k)
}

pub fn derivative(c: &[f64]) -> Vec<f64> {
    c.iter().enumerate().skip(1).map(|(i, &k)| k * i as f64).collect()
}

pub fn add(a: &[f64], b: &[f64]) -> Vec<f64> {
    let mut r = vec![0.; a.len().max(b.len())];
    for (i, k) in a.iter().enumerate() { r[i] += k; }
    for (i, k) in b.iter().enumerate() { r[i] += k; }
    r
}

pub fn scale(a: &[f64], s: f64) -> Vec<f64> {
    a.iter().map(|k| k * s).collect()
}

pub fn mul(a: &[f64], b: &[f64]) -> Vec<f64> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut r = vec![0.; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            r[i + j] += x * y;
        }
    }
    r
}

/// `a` raised to the power `n`.
pub fn pow(a: &[f64], n: usize) -> Vec<f64> {
    (0..n).fold(vec![1.], |acc, _| mul(&acc, a))
}

/// `p(q(t))`: substitute one polynomial into another.
pub fn compose(p: &[f64], q: &[f64]) -> Vec<f64> {
    p.iter().rev().fold(vec![], |acc, &k| add(&mul(&acc, q), &[k]))
}

/// Drop leading coefficients that are negligible beside the largest, so that the degree is the
/// effective one (eg. a quadratic whose t^2 term vanishes is solved as a line).
fn trim(c: &[f64]) -> &[f64] {
    let max = c.iter().fold(0f64, |m, k| m.max(k.abs()));
    if max == 0. {
        return &c[..0];
    }
    let mut n = c.len();
    while n > 0 && c[n - 1].abs() <= max * 1e-12 {
        n -= 1;
    }
    &c[..n]
}

/// The root in [lo, hi] of a polynomial that is monotonic there, given values of opposite sign
/// at the ends: bisection, which always converges.
fn bracket(c: &[f64], mut lo: f64, mut hi: f64, mut flo: f64) -> f64 {
    for _ in 0..100 {
        let mid = 0.5 * (lo + hi);
        if mid <= lo || mid >= hi {
            break;
        }
        let fm = eval(c, mid);
        if fm == 0. {
            return mid;
        }
        if (fm < 0.) == (flo < 0.) {
            lo = mid;
            flo = fm;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

/// The real roots of the polynomial in [lo, hi], in increasing order.
pub fn roots_in(c: &[f64], lo: f64, hi: f64) -> Vec<f64> {
    let c = trim(c);
    // Also rejects NaN limits.
    if lo.partial_cmp(&hi).is_none_or(|o| o == std::cmp::Ordering::Greater) {
        return vec![];
    }
    match c.len() {
        0 | 1 => return vec![],
        2 => {
            let t = -c[0] / c[1];
            return if t >= lo && t <= hi { vec![t] } else { vec![] };
        },
        3 => return quadratic_roots(c[0], c[1], c[2]).into_iter().filter(|t| *t >= lo && *t <= hi).collect(),
        _ => {}
    }
    // Split at the critical points, inside which the polynomial is monotonic.
    let mut edges = vec![lo];
    edges.extend(roots_in(&derivative(c), lo, hi));
    edges.push(hi);
    let mut roots = Vec::new();
    for w in edges.windows(2) {
        let (a, b) = (w[0], w[1]);
        let (fa, fb) = (eval(c, a), eval(c, b));
        if fa == 0. {
            if roots.last() != Some(&a) { roots.push(a); }
        } else if fb == 0. {
            roots.push(b);
        } else if (fa < 0.) != (fb < 0.) {
            roots.push(bracket(c, a, b, fa));
        }
    }
    roots
}

/// The real roots of c + b t + a t^2, in increasing order, avoiding cancellation (Press et al.,
/// "Numerical Recipes", 3rd ed., section 5.6).
pub fn quadratic_roots(c: f64, b: f64, a: f64) -> Vec<f64> {
    if a == 0. {
        return if b == 0. { vec![] } else { vec![-c / b] };
    }
    let d = b * b - 4. * a * c;
    if d < 0. {
        return vec![];
    }
    let q = -0.5 * (b + b.signum() * d.sqrt());
    if q == 0. {
        return vec![0.];
    }
    let (r1, r2) = (q / a, c / q);
    if r1 < r2 { vec![r1, r2] } else { vec![r2, r1] }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from_roots(rs: &[f64]) -> Vec<f64> {
        rs.iter().fold(vec![1.], |acc, r| mul(&acc, &[-r, 1.]))
    }

    #[test]
    fn test_finds_all_roots_of_a_quartic() {
        let c = from_roots(&[-2., 0.5, 1., 3.]);
        let r = roots_in(&c, -10., 10.);
        assert_eq!(r.len(), 4);
        for (a, b) in r.iter().zip([-2., 0.5, 1., 3.]) {
            assert!((a - b).abs() < 1e-9, "{:?}", r);
        }
        assert_eq!(roots_in(&c, 0., 2.).len(), 2);
    }

    #[test]
    fn test_high_degree() {
        let rs = [-3., -1.5, -0.2, 0.1, 0.7, 1.9];
        let r = roots_in(&from_roots(&rs), -5., 5.);
        assert_eq!(r.len(), 6);
        for (a, b) in r.iter().zip(rs) {
            assert!((a - b).abs() < 1e-8);
        }
    }

    #[test]
    fn test_no_real_roots() {
        // t^4 + 1
        assert!(roots_in(&[1., 0., 0., 0., 1.], -10., 10.).is_empty());
    }

    #[test]
    fn test_degenerate_leading_term() {
        // 1e-20 t^3 + t - 2: effectively linear.
        let r = roots_in(&[-2., 1., 0., 1e-20], -10., 10.);
        assert_eq!(r.len(), 1);
        assert!((r[0] - 2.).abs() < 1e-9);
    }

    #[test]
    fn test_compose() {
        // (t + 1)^2 = t^2 + 2t + 1
        assert_eq!(compose(&[0., 0., 1.], &[1., 1.]), vec![1., 2., 1.]);
        assert_eq!(pow(&[1., 1.], 2), vec![1., 2., 1.]);
    }
}
