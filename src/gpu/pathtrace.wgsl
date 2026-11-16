// The GPU path tracer: one invocation per pixel per pass, over the flattened scene (see
// `gpu/mod.rs` for the buffers' layouts). It ports the CPU wavefront integrator's shading
// (`wavefront::integrator`): the same BSDFs, light sampling with the power heuristic, Russian
// roulette and regularisation, in f32.

struct Params {
    dims: vec4u,     // width, height, pass, max_depth
    counts: vec4u,   // prims (bounded), planes, emitters, lights
    offs: vec4u,     // emitter prims, emitter cdf, env radiance, env marginal cdf (in tables)
    offs2: vec4u,    // env conditional cdf, env width, env height, flags
    misc: vec4f,     // regularise, black threshold, shadow bias, russian roulette min depth
    sun_dir: vec4f,  // direction, cos of angular radius
    sun_irr: vec4f,  // irradiance, angular radius
    seed: vec4u,     // render seed, russian roulette on, aerial perspective table, -
    ap_x: vec4f,     // the sun's azimuth, for the aerial perspective table
    ap_z: vec4f,
}

const FLAG_SAMPLE_LIGHTS: u32 = 1u;
const FLAG_SAMPLE_ENV: u32 = 2u;
const FLAG_SUN: u32 = 4u;
const FLAG_AERIAL: u32 = 8u;
const AP_AZIMUTH: u32 = 32u;
const AP_ELEVATION: u32 = 64u;
const AP_DISTANCE: u32 = 32u;
const AERIAL_FROM: f32 = 100.0;
const AERIAL_TO: f32 = 2e5;

struct Prim {
    v0: vec4f,       // triangle vertex, or sphere centre and radius, or plane y in x
    v1: vec4f,
    v2: vec4f,
    n0: vec4f,       // vertex normals (smooth triangles)
    n1: vec4f,
    n2: vec4f,
    info: vec4u,     // kind (0 triangle, 1 sphere, 2 plane), material, emitter + 1 (0 none), smooth
}

struct Node {
    bmin: vec3f,
    offset: u32,     // leaf: first item; interior: second child
    bmax: vec3f,
    count: u32,      // leaf: items; interior: 0
}

struct Material {
    kind: u32,       // 0 diffuse, 1 plastic, 2 dielectric, 3 conductor, 4 emitter, 5 metal, 6 checker
    a: u32,          // checker: first material; emitter: two-sided
    b: u32,          // checker: second material
    pad: u32,
    c0: vec4f,       // albedo, tint, radiance, or conductor's eta
    c1: vec4f,       // conductor's k
    p: vec4f,        // refractive index, roughness, checker x size, z size
}

struct Light {
    pos: vec4f,      // position, kind (0 point, 1 spot)
    intensity: vec4f,
    axis: vec4f,     // spot axis, cos at the start of the falloff
    extra: vec4f,    // cos at the edge
}

struct CameraRay {
    o: vec4f,        // origin, weight
    d: vec4f,        // direction, 1 if valid
}

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> prims: array<Prim>;
@group(0) @binding(2) var<storage, read> nodes: array<Node>;
@group(0) @binding(3) var<storage, read> materials: array<Material>;
@group(0) @binding(4) var<storage, read> lights: array<Light>;
@group(0) @binding(5) var<storage, read> tables: array<f32>;
@group(0) @binding(6) var<storage, read> rays: array<CameraRay>;
@group(0) @binding(7) var<storage, read_write> accum: array<vec4f>;

const PI: f32 = 3.14159265358979;
const MIN_ALPHA: f32 = 1e-3;
const INF: f32 = 3.0e38;
const LIMB_DARKENING: f32 = 0.6;

// ---- Random numbers: a counter-based hash (PCG), per pixel, pass and dimension. ----

var<private> rng_state: u32;

fn pcg(v: u32) -> u32 {
    let state = v * 747796405u + 2891336453u;
    let word = ((state >> ((state >> 28u) + 4u)) ^ state) * 277803737u;
    return (word >> 22u) ^ word;
}

fn rand() -> f32 {
    rng_state = pcg(rng_state);
    return f32(rng_state >> 8u) * (1.0 / 16777216.0);
}

// ---- Geometry. ----

struct Hit {
    t: f32,
    prim: u32,
    ng: vec3f,
    ns: vec3f,
}

fn intersect_prim(i: u32, ro: vec3f, rd: vec3f, tmin: f32, tmax: f32) -> vec4f {
    // Returns (t, b1, b2, ok).
    let p = prims[i];
    let kind = p.info.x;
    if (kind == 0u) {
        let e1 = p.v1.xyz - p.v0.xyz;
        let e2 = p.v2.xyz - p.v0.xyz;
        let pv = cross(rd, e2);
        let det = dot(e1, pv);
        if (abs(det) < 1e-12) { return vec4f(0.0); }
        let inv = 1.0 / det;
        let s = ro - p.v0.xyz;
        let b1 = dot(s, pv) * inv;
        if (b1 < 0.0 || b1 > 1.0) { return vec4f(0.0); }
        let q = cross(s, e1);
        let b2 = dot(rd, q) * inv;
        if (b2 < 0.0 || b1 + b2 > 1.0) { return vec4f(0.0); }
        let t = dot(e2, q) * inv;
        if (t <= tmin || t >= tmax) { return vec4f(0.0); }
        return vec4f(t, b1, b2, 1.0);
    } else if (kind == 1u) {
        let oc = ro - p.v0.xyz;
        let b = dot(oc, rd);
        let c = dot(oc, oc) - p.v0.w * p.v0.w;
        let disc = b * b - c;
        if (disc < 0.0) { return vec4f(0.0); }
        let sq = sqrt(disc);
        var t = -b - sq;
        if (t <= tmin || t >= tmax) { t = -b + sq; }
        if (t <= tmin || t >= tmax) { return vec4f(0.0); }
        return vec4f(t, 0.0, 0.0, 1.0);
    } else {
        if (abs(rd.y) < 1e-12) { return vec4f(0.0); }
        let t = (p.v0.x - ro.y) / rd.y;
        if (t <= tmin || t >= tmax) { return vec4f(0.0); }
        return vec4f(t, 0.0, 0.0, 1.0);
    }
}

fn normals(i: u32, ro: vec3f, rd: vec3f, h: vec4f) -> array<vec3f, 2> {
    let p = prims[i];
    let kind = p.info.x;
    if (kind == 0u) {
        let g = normalize(cross(p.v1.xyz - p.v0.xyz, p.v2.xyz - p.v0.xyz));
        var s = g;
        if (p.info.w == 1u) {
            let n = p.n0.xyz * (1.0 - h.y - h.z) + p.n1.xyz * h.y + p.n2.xyz * h.z;
            if (dot(n, n) > 0.0) { s = normalize(n); }
        }
        return array<vec3f, 2>(g, s);
    } else if (kind == 1u) {
        let n = (ro + rd * h.x - p.v0.xyz) / p.v0.w;
        return array<vec3f, 2>(n, n);
    }
    return array<vec3f, 2>(vec3f(0.0, 1.0, 0.0), vec3f(0.0, 1.0, 0.0));
}

fn box_entry(bmin: vec3f, bmax: vec3f, ro: vec3f, inv: vec3f, tmax: f32) -> f32 {
    let t1 = (bmin - ro) * inv;
    let t2 = (bmax - ro) * inv;
    let near = max(max(min(t1.x, t2.x), min(t1.y, t2.y)), min(t1.z, t2.z));
    let far = min(min(max(t1.x, t2.x), max(t1.y, t2.y)), max(t1.z, t2.z));
    if (far < 0.0 || near > far || near > tmax) { return INF; }
    return max(near, 0.0);
}

// The nearest hit beyond `tmin`, or (with `any`) whether anything is hit before `tmax`.
fn trace(ro: vec3f, rd: vec3f, tmin: f32, tmax_in: f32, any_hit: bool) -> Hit {
    var best = Hit(tmax_in, 0xffffffffu, vec3f(0.0), vec3f(0.0));
    var best_h = vec4f(0.0);
    let np = params.counts.x;
    // Planes, after the bounded primitives.
    for (var k = 0u; k < params.counts.y; k++) {
        let h = intersect_prim(np + k, ro, rd, tmin, best.t);
        if (h.w > 0.0) {
            best.t = h.x; best.prim = np + k; best_h = h;
            if (any_hit) { return best; }
        }
    }
    if (np > 0u) {
        let inv = 1.0 / rd;
        var stack: array<u32, 100>;
        var sp = 0u;
        stack[0] = 0u;
        sp = 1u;
        loop {
            if (sp == 0u) { break; }
            sp -= 1u;
            let ni = stack[sp];
            let n = nodes[ni];
            if (box_entry(n.bmin, n.bmax, ro, inv, best.t) == INF) { continue; }
            if (n.count > 0u) {
                for (var j = n.offset; j < n.offset + n.count; j++) {
                    let h = intersect_prim(j, ro, rd, tmin, best.t);
                    if (h.w > 0.0) {
                        best.t = h.x; best.prim = j; best_h = h;
                        if (any_hit) { return best; }
                    }
                }
            } else if (sp + 2u <= 100u) {
                stack[sp] = n.offset; sp += 1u;
                stack[sp] = ni + 1u; sp += 1u;
            }
        }
    }
    if (best.prim != 0xffffffffu) {
        let ns = normals(best.prim, ro, rd, best_h);
        best.ng = ns[0];
        best.ns = ns[1];
    }
    return best;
}

fn occluded(ro: vec3f, rd: vec3f, tmax: f32) -> bool {
    return trace(ro, rd, 0.0, tmax, true).prim != 0xffffffffu;
}

// ---- Tables: emitters and the environment. ----

fn table_u32(i: u32) -> u32 { return bitcast<u32>(tables[i]); }

// The index i with cdf[i] <= u < cdf[i + 1], over n pieces starting at `off`.
fn search(off: u32, n: u32, u: f32) -> u32 {
    var lo = 0u;
    var hi = n;
    while (lo + 1u < hi) {
        let mid = (lo + hi) / 2u;
        if (tables[off + mid] <= u) { lo = mid; } else { hi = mid; }
    }
    return lo;
}

fn emitter_probability(e: u32) -> f32 {
    let off = params.offs.y;
    return tables[off + e + 1u] - tables[off + e];
}

fn env_dims() -> vec2u { return vec2u(params.offs2.y, params.offs2.z); }

fn env_texel(x: u32, y: u32) -> vec3f {
    let dims = env_dims();
    let i = params.offs.z + 3u * (y * dims.x + (x % dims.x));
    return vec3f(tables[i], tables[i + 1u], tables[i + 2u]);
}

// The baked sky in direction d, bilinearly interpolated (around in azimuth, clamped at the
// poles).
fn env_lookup(d: vec3f) -> vec3f {
    let dims = env_dims();
    let u = fract(atan2(d.z, d.x) / (2.0 * PI) + 0.5) * f32(dims.x) - 0.5;
    let v = clamp(acos(clamp(d.y, -1.0, 1.0)) / PI * f32(dims.y) - 0.5, 0.0, f32(dims.y - 1u));
    let x0 = floor(u);
    let y0 = floor(v);
    let fx = u - x0;
    let fy = v - y0;
    let xi = u32(i32(x0) + i32(dims.x)) % dims.x;
    let yi = u32(y0);
    let y1 = min(yi + 1u, dims.y - 1u);
    let top = mix(env_texel(xi, yi), env_texel(xi + 1u, yi), fx);
    let bottom = mix(env_texel(xi, y1), env_texel(xi + 1u, y1), fx);
    return mix(top, bottom, fy);
}

fn env_pdf(d: vec3f) -> f32 {
    let dims = env_dims();
    let u = fract(atan2(d.z, d.x) / (2.0 * PI) + 0.5);
    let v = acos(clamp(d.y, -1.0, 1.0)) / PI;
    let x = min(u32(u * f32(dims.x)), dims.x - 1u);
    let y = min(u32(v * f32(dims.y)), dims.y - 1u);
    let sin_theta = sin(v * PI);
    if (sin_theta <= 0.0) { return 0.0; }
    let marg = params.offs.w;
    let cond = params.offs2.x + y * (dims.x + 1u);
    let p_row = (tables[marg + y + 1u] - tables[marg + y]) * f32(dims.y);
    let p_col = (tables[cond + x + 1u] - tables[cond + x]) * f32(dims.x);
    return p_row * p_col / (2.0 * PI * PI * sin_theta);
}

// A direction org the environment, in proportion to its brightness, and its density.
fn env_sample(u1: f32, u2: f32) -> vec4f {
    let dims = env_dims();
    let y = search(params.offs.w, dims.y, u1);
    let x = search(params.offs2.x + y * (dims.x + 1u), dims.x, u2);
    let u = (f32(x) + 0.5) / f32(dims.x);
    let v = (f32(y) + 0.5) / f32(dims.y);
    let phi = (u - 0.5) * 2.0 * PI;
    let theta = v * PI;
    let d = vec3f(sin(theta) * cos(phi), cos(theta), sin(theta) * sin(phi));
    return vec4f(d, env_pdf(d));
}

// ---- Scattering. ----

fn reflect_about(v: vec3f, n: vec3f) -> vec3f { return v - n * 2.0 * dot(n, v); }

fn schlick(cosine: f32, ior: f32) -> f32 {
    let r0 = pow((1.0 - ior) / (1.0 + ior), 2.0);
    return r0 + (1.0 - r0) * pow(1.0 - cosine, 5.0);
}

// `functions::fresnel_reflectance`.
fn fresnel_reflectance(cos_i: f32, ni_over_nt: f32, ior: f32) -> f32 {
    let sin2_t = ni_over_nt * ni_over_nt * (1.0 - cos_i * cos_i);
    if (sin2_t >= 1.0) { return 1.0; }
    let cos_t = sqrt(1.0 - sin2_t);
    return schlick(min(cos_i, cos_t), ior);
}

// `ggx::conductor_reflectance`, with complex arithmetic spelled out.
fn conductor_reflectance(cos_in: f32, eta: f32, k: f32) -> f32 {
    let cos_i = clamp(cos_in, 0.0, 1.0);
    let sin2_i = 1.0 - cos_i * cos_i;
    // n² = (eta + ik)²
    let n2 = vec2f(eta * eta - k * k, 2.0 * eta * k);
    // sin²t = sin²i / n²
    let d = dot(n2, n2);
    let s2t = vec2f(sin2_i * n2.x / d, -sin2_i * n2.y / d);
    // cos t = sqrt(1 - sin²t), principal root
    let z = vec2f(1.0 - s2t.x, -s2t.y);
    let r = length(z);
    var ct = vec2f(sqrt(max((r + z.x) / 2.0, 0.0)), sqrt(max((r - z.x) / 2.0, 0.0)));
    if (z.y < 0.0) { ct.y = -ct.y; }
    let n = vec2f(eta, k);
    // r_parl = (n cos_i - cos_t) / (n cos_i + cos_t); r_perp = (cos_i - n cos_t) / (cos_i + n cos_t)
    let a = n * cos_i;
    let num1 = a - ct;
    let den1 = a + ct;
    let nct = vec2f(n.x * ct.x - n.y * ct.y, n.x * ct.y + n.y * ct.x);
    let num2 = vec2f(cos_i - nct.x, -nct.y);
    let den2 = vec2f(cos_i + nct.x, nct.y);
    let rp = dot(num1, num1) / max(dot(den1, den1), 1e-30);
    let rs = dot(num2, num2) / max(dot(den2, den2), 1e-30);
    return (rp + rs) / 2.0;
}

// Fresnel for a GGX lobe: kind 0 Schlick (c0 = F0), 1 dielectric (ior), 2 conductor (c0 eta, c1 k).
struct Lobe {
    kind: u32,       // 0 lambert, 1 ggx, 2 plastic (ggx coat + lambert base)
    fresnel: u32,
    alpha: f32,
    ior: f32,
    albedo: vec3f,   // lambert albedo, Schlick F0, or eta
    k: vec3f,
    n: vec3f,
    t: vec3f,
    b: vec3f,
}

fn fresnel_eval(l: Lobe, cos: f32) -> vec3f {
    if (l.fresnel == 0u) {
        let k = pow(max(1.0 - cos, 0.0), 5.0);
        return l.albedo * (1.0 - k) + vec3f(k);
    } else if (l.fresnel == 1u) {
        return vec3f(fresnel_reflectance(cos, 1.0 / l.ior, l.ior));
    }
    return vec3f(conductor_reflectance(cos, l.albedo.x, l.k.x), conductor_reflectance(cos, l.albedo.y, l.k.y),
                 conductor_reflectance(cos, l.albedo.z, l.k.z));
}

fn basis(n: vec3f) -> array<vec3f, 2> {
    // Duff et al.'s orthonormal basis.
    let s = select(-1.0, 1.0, n.z >= 0.0);
    let a = -1.0 / (s + n.z);
    let b = n.x * n.y * a;
    return array<vec3f, 2>(vec3f(1.0 + s * n.x * n.x * a, s * b, -s * n.x), vec3f(b, s + n.y * n.y * a, -n.y));
}

fn make_lobe(kind: u32, n: vec3f) -> Lobe {
    let tb = basis(n);
    return Lobe(kind, 0u, 1.0, 1.5, vec3f(0.0), vec3f(0.0), n, tb[0], tb[1]);
}

fn to_local(l: Lobe, v: vec3f) -> vec3f { return vec3f(dot(v, l.t), dot(v, l.b), dot(v, l.n)); }
fn to_world(l: Lobe, v: vec3f) -> vec3f { return l.t * v.x + l.b * v.y + l.n * v.z; }

fn ggx_d(alpha: f32, m: vec3f) -> f32 {
    if (m.z <= 0.0) { return 0.0; }
    let a2 = alpha * alpha;
    let k = (a2 - 1.0) * m.z * m.z + 1.0;
    return a2 / (PI * k * k);
}

fn ggx_lambda(alpha: f32, w: vec3f) -> f32 {
    let cos2 = w.z * w.z;
    if (cos2 >= 1.0) { return 0.0; }
    let a2tan2 = alpha * alpha * (w.x * w.x + w.y * w.y) / cos2;
    return (sqrt(1.0 + a2tan2) - 1.0) / 2.0;
}

fn ggx_eval(l: Lobe, wo: vec3f, wi: vec3f) -> vec3f {
    let o = to_local(l, wo);
    let i = to_local(l, wi);
    if (o.z <= 0.0 || i.z <= 0.0) { return vec3f(0.0); }
    let h = normalize(o + i);
    let g2 = 1.0 / (1.0 + ggx_lambda(l.alpha, o) + ggx_lambda(l.alpha, i));
    return fresnel_eval(l, dot(o, h)) * (g2 * ggx_d(l.alpha, h) / (4.0 * o.z));
}

fn ggx_pdf(l: Lobe, wo: vec3f, wi: vec3f) -> f32 {
    let o = to_local(l, wo);
    let i = to_local(l, wi);
    if (o.z <= 0.0 || i.z <= 0.0) { return 0.0; }
    let h = normalize(o + i);
    return ggx_d(l.alpha, h) / (1.0 + ggx_lambda(l.alpha, o)) / (4.0 * o.z);
}

// Heitz's sampling of visible normals (`GGX::sample_visible_normal`).
fn ggx_sample(l: Lobe, wo: vec3f, u1: f32, u2: f32) -> vec3f {
    let o = to_local(l, wo);
    if (o.z <= 0.0) { return vec3f(0.0); }
    let vh = normalize(vec3f(l.alpha * o.x, l.alpha * o.y, o.z));
    let lensq = vh.x * vh.x + vh.y * vh.y;
    var t1 = vec3f(1.0, 0.0, 0.0);
    if (lensq > 0.0) { t1 = vec3f(-vh.y, vh.x, 0.0) / sqrt(lensq); }
    let t2 = cross(vh, t1);
    let r = sqrt(u1);
    let phi = 2.0 * PI * u2;
    let p1 = r * cos(phi);
    let s = 0.5 * (1.0 + vh.z);
    let p2 = (1.0 - s) * sqrt(max(1.0 - p1 * p1, 0.0)) + s * r * sin(phi);
    let nh = t1 * p1 + t2 * p2 + vh * sqrt(max(1.0 - p1 * p1 - p2 * p2, 0.0));
    let m = normalize(vec3f(l.alpha * nh.x, l.alpha * nh.y, max(nh.z, 0.0)));
    let i = m * (2.0 * dot(o, m)) - o;
    if (i.z <= 0.0) { return vec3f(0.0); }
    return to_world(l, i);
}

fn lambert_eval(l: Lobe, wi: vec3f) -> vec3f {
    let c = dot(wi, l.n);
    if (c <= 0.0) { return vec3f(0.0); }
    return l.albedo * (c / PI);
}

fn coat_share(l: Lobe, wo: vec3f) -> f32 {
    return fresnel_reflectance(max(dot(wo, l.n), 0.0), 1.0 / l.ior, l.ior);
}

// f |cos θ_i|, for a lobe.
fn bsdf_eval(l: Lobe, base: vec3f, wo: vec3f, wi: vec3f) -> vec3f {
    if (l.kind == 0u) { return lambert_eval(l, wi); }
    if (l.kind == 1u) { return ggx_eval(l, wo, wi); }
    // Plastic: coat plus base, the base dimmed by what the coat reflects.
    var lb = l; lb.albedo = base;
    var lc = l; lc.albedo = vec3f(0.0);
    return ggx_eval(lc, wo, wi) + lambert_eval(lb, wi) * (1.0 - coat_share(l, wo));
}

fn bsdf_pdf(l: Lobe, wo: vec3f, wi: vec3f) -> f32 {
    let cos_pdf = max(dot(wi, l.n), 0.0) / PI;
    if (l.kind == 0u) { return cos_pdf; }
    if (l.kind == 1u) { return ggx_pdf(l, wo, wi); }
    let f = coat_share(l, wo);
    return f * ggx_pdf(l, wo, wi) + (1.0 - f) * cos_pdf;
}

fn uniform_sphere(u: f32, v: f32) -> vec3f {
    let theta = u * 2.0 * PI;
    let z = 2.0 * v - 1.0;
    let s = sqrt(max(1.0 - z * z, 0.0));
    return vec3f(s * cos(theta), s * sin(theta), z);
}

fn cosine_sample(n: vec3f, u: f32, v: f32) -> vec3f {
    let d = n + uniform_sphere(u, v);
    if (length(d) < 1e-6) { return n; }
    return normalize(d);
}

// A direction (xyz) and its density (w); w = 0 if none.
fn bsdf_sample(l: Lobe, wo: vec3f, u1: f32, u2: f32, u3: f32) -> vec4f {
    var wi: vec3f;
    if (l.kind == 0u) {
        wi = cosine_sample(l.n, u1, u2);
    } else if (l.kind == 1u) {
        wi = ggx_sample(l, wo, u1, u2);
    } else if (u3 < coat_share(l, wo)) {
        wi = ggx_sample(l, wo, u1, u2);
    } else {
        wi = cosine_sample(l.n, u1, u2);
    }
    if (dot(wi, wi) == 0.0) { return vec4f(0.0); }
    return vec4f(wi, bsdf_pdf(l, wo, wi));
}

fn power_heuristic(p: f32, q: f32) -> f32 {
    if (p >= INF) { return 1.0; }
    let p2 = p * p;
    let q2 = q * q;
    if (p2 + q2 == 0.0) { return 0.0; }
    return p2 / (p2 + q2);
}

fn luminance(c: vec3f) -> f32 { return dot(c, vec3f(0.2126, 0.7152, 0.0722)); }

fn resolve_material(m_in: u32, p: vec3f) -> u32 {
    var m = m_in;
    for (var k = 0; k < 4; k++) {
        let mat = materials[m];
        if (mat.kind != 6u) { break; }
        let zig = select(p.x <= 0.0, p.x > 0.0, (u32(abs(p.x) / mat.p.z) % 2u) == 0u);
        let zag = select(p.z <= 0.0, p.z > 0.0, (u32(abs(p.z) / mat.p.w) % 2u) == 0u);
        m = select(mat.b, mat.a, zig != zag);
    }
    return m;
}

fn emitted(m: u32, n: vec3f, towards: vec3f) -> vec3f {
    let mat = materials[m];
    if (mat.kind != 4u) { return vec3f(0.0); }
    if (mat.a == 1u || dot(towards, n) > 0.0) { return mat.c0.xyz; }
    return vec3f(0.0);
}

fn prim_area(i: u32) -> f32 {
    let p = prims[i];
    if (p.info.x == 0u) { return length(cross(p.v1.xyz - p.v0.xyz, p.v2.xyz - p.v0.xyz)) / 2.0; }
    return 4.0 * PI * p.v0.w * p.v0.w;
}

// A point on emitting primitive i seen org `org`, its normal, and the density per solid
// angle (a sphere by its cone, a triangle by area), as `wavefront::integrator::sample_light`.
struct LightSample { y: vec3f, n: vec3f, pdf: f32 }

fn sphere_pdf(c: vec3f, r: f32, org: vec3f, y: vec3f, n: vec3f) -> f32 {
    let d2 = dot(c - org, c - org);
    let r2 = r * r;
    if (d2 <= r2 * 1.000001) {
        let w = y - org;
        let dist2 = dot(w, w);
        let cos = abs(dot(n, w)) / sqrt(dist2);
        return dist2 / (cos * 4.0 * PI * r2);
    }
    let cos_max = sqrt(max(1.0 - r2 / d2, 0.0));
    return 1.0 / (2.0 * PI * (1.0 - cos_max));
}

fn sample_light(i: u32, org: vec3f, u1: f32, u2: f32) -> LightSample {
    let p = prims[i];
    if (p.info.x == 1u) {
        let c = p.v0.xyz;
        let r = p.v0.w;
        let to = c - org;
        let d2 = dot(to, to);
        if (d2 <= r * r * 1.000001) {
            let n = uniform_sphere(u1, u2);
            let y = c + n * r;
            return LightSample(y, n, sphere_pdf(c, r, org, y, n));
        }
        let d = sqrt(d2);
        let w = to / d;
        let sin2_max = r * r / d2;
        let cos_max = sqrt(max(1.0 - sin2_max, 0.0));
        let cos_t = 1.0 - u1 * (1.0 - cos_max);
        let sin_t = sqrt(max(1.0 - cos_t * cos_t, 0.0));
        let phi = 2.0 * PI * u2;
        let tb = basis(w);
        let dir = tb[0] * (sin_t * cos(phi)) + tb[1] * (sin_t * sin(phi)) + w * cos_t;
        // Where that direction meets the sphere, nearest first.
        let b = dot(org - c, dir);
        let disc = max(b * b - (d2 - r * r), 0.0);
        let t = -b - sqrt(disc);
        let y = org + dir * t;
        let n = normalize(y - c);
        return LightSample(y, n, 1.0 / (2.0 * PI * (1.0 - cos_max)));
    }
    let su = sqrt(u1);
    let b1 = 1.0 - su;
    let b2 = u2 * su;
    let y = p.v0.xyz * b1 + p.v1.xyz * b2 + p.v2.xyz * (1.0 - b1 - b2);
    let n = normalize(cross(p.v1.xyz - p.v0.xyz, p.v2.xyz - p.v0.xyz));
    let w = y - org;
    let d2 = dot(w, w);
    let cos = abs(dot(n, w)) / sqrt(d2);
    var pdf = 0.0;
    if (cos > 0.0) { pdf = d2 / (cos * prim_area(i)); }
    return LightSample(y, n, pdf);
}

fn light_pdf(i: u32, org: vec3f, y: vec3f, n: vec3f) -> f32 {
    let p = prims[i];
    if (p.info.x == 1u) { return sphere_pdf(p.v0.xyz, p.v0.w, org, y, n); }
    let w = y - org;
    let d2 = dot(w, w);
    let cos = abs(dot(n, w)) / sqrt(d2);
    if (cos <= 0.0) { return 0.0; }
    return d2 / (cos * prim_area(i));
}

fn sun_disc(d: vec3f) -> vec3f {
    let cos = dot(d, params.sun_dir.xyz);
    let cos_r = params.sun_dir.w;
    if (cos <= cos_r) { return vec3f(0.0); }
    let theta = acos(min(cos, 1.0));
    let mu = sqrt(max(1.0 - pow(theta / params.sun_irr.w, 2.0), 0.0));
    let solid_angle = 2.0 * PI * (1.0 - cos_r);
    return params.sun_irr.xyz * ((1.0 - LIMB_DARKENING * (1.0 - mu)) / (1.0 - LIMB_DARKENING / 3.0) / solid_angle);
}

fn sun_disc_pdf(d: vec3f) -> f32 {
    if (dot(d, params.sun_dir.xyz) <= params.sun_dir.w) { return 0.0; }
    return 1.0 / (2.0 * PI * (1.0 - params.sun_dir.w));
}

fn ap_entry(a: u32, e: u32, k: u32) -> array<vec3f, 2> {
    let i = params.seed.z + 6u * ((a * AP_ELEVATION + e) * AP_DISTANCE + k);
    return array<vec3f, 2>(vec3f(tables[i], tables[i + 1u], tables[i + 2u]), vec3f(tables[i + 3u], tables[i + 4u], tables[i + 5u]));
}

// Transmittance and inscattered light along d to distance t (trilinear in the table).
fn aerial(d: vec3f, t: f32) -> array<vec3f, 2> {
    let x = dot(d, params.ap_x.xyz);
    let z = dot(d, params.ap_z.xyz);
    let fa = clamp(abs(atan2(z, x)) / PI, 0.0, 1.0) * f32(AP_AZIMUTH - 1u);
    let fe = clamp((d.y + 1.0) / 2.0, 0.0, 1.0) * f32(AP_ELEVATION - 1u);
    let fk = clamp(log(t / AERIAL_FROM) / log(AERIAL_TO / AERIAL_FROM), 0.0, 1.0) * f32(AP_DISTANCE - 1u);
    let a0 = min(u32(fa), AP_AZIMUTH - 2u);
    let e0 = min(u32(fe), AP_ELEVATION - 2u);
    let k0 = min(u32(fk), AP_DISTANCE - 2u);
    let wa = fa - f32(a0);
    let we = fe - f32(e0);
    let wk = fk - f32(k0);
    var tr = vec3f(0.0);
    var ins = vec3f(0.0);
    for (var c = 0u; c < 8u; c++) {
        let da = c & 1u;
        let de = (c >> 1u) & 1u;
        let dk = (c >> 2u) & 1u;
        let w = select(1.0 - wa, wa, da == 1u) * select(1.0 - we, we, de == 1u) * select(1.0 - wk, wk, dk == 1u);
        let v = ap_entry(a0 + da, e0 + de, k0 + dk);
        tr += v[0] * w;
        ins += v[1] * w;
    }
    return array<vec3f, 2>(tr, ins);
}

fn spot(light: Light, towards: vec3f) -> f32 {
    if (light.pos.w < 0.5) { return 1.0; }
    let a = light.extra.x;
    let b = light.axis.w;
    let x = dot(light.axis.xyz, towards);
    if (a == b) { return select(1.0, 0.0, x < a); }
    let t = clamp((x - a) / (b - a), 0.0, 1.0);
    return t * t * (3.0 - 2.0 * t);
}

fn offset_for(p: vec3f) -> f32 {
    let m = max(max(abs(p.x), abs(p.y)), abs(p.z));
    return max(params.misc.z, 2e-5 * (1.0 + m)) * 2.0;
}

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3u) {
    let width = params.dims.x;
    let height = params.dims.y;
    // Dispatched as rows of at most 65535 workgroups.
    let index = gid.x + gid.y * (65535u * 64u);
    if (index >= width * height) { return; }
    let cam = rays[index];
    if (cam.d.w < 0.5) { return; }
    rng_state = pcg(index ^ pcg(params.dims.z * 0x9e3779b9u ^ params.seed.x));
    var ro = cam.o.xyz;
    var rd = normalize(cam.d.xyz);
    var beta = vec3f(1.0);
    var radiance = vec3f(0.0);
    var specular = true;
    var prev_pdf = 0.0;
    var scattered = false;
    let max_depth = params.dims.w;
    let flags = params.offs2.w;
    let n_emitters = params.counts.z;
    var depth = 0u;
    loop {
        let hit = trace(ro, rd, 0.0, INF, false);
        if (hit.prim == 0xffffffffu) {
            // The sky, weighted against sampling it if the last vertex did.
            var w = 1.0;
            if (!specular && (flags & FLAG_SAMPLE_ENV) != 0u) { w = power_heuristic(prev_pdf, env_pdf(rd)); }
            radiance += beta * env_lookup(rd) * w;
            if ((flags & FLAG_SUN) != 0u) {
                var ws = 1.0;
                if (!specular) { ws = power_heuristic(prev_pdf, sun_disc_pdf(rd)); }
                radiance += beta * sun_disc(rd) * ws;
            }
            break;
        }
        // Aerial perspective, as the CPU's: the air between attenuates what lies beyond the hit,
        // and adds its own light.
        if ((flags & FLAG_AERIAL) != 0u && hit.t > AERIAL_FROM) {
            let ap = aerial(rd, hit.t);
            radiance += beta * ap[1];
            beta *= ap[0];
        }
        let point = ro + rd * hit.t;
        let prim = prims[hit.prim];
        let m = resolve_material(prim.info.y, point);
        let mat = materials[m];
        if (mat.kind == 4u) {
            let le = emitted(m, hit.ng, -rd);
            var w = 1.0;
            if (!specular && (flags & FLAG_SAMPLE_LIGHTS) != 0u && prim.info.z > 0u) {
                let e = prim.info.z - 1u;
                let lp = emitter_probability(e) * light_pdf(hit.prim, ro, point, hit.ng);
                w = power_heuristic(prev_pdf, lp);
            }
            radiance += beta * le * w;
            break;
        }
        let wo = -rd;
        var ng = hit.ng;
        if (dot(ng, wo) < 0.0) { ng = -ng; }
        var ns = hit.ns;
        if (dot(ns, wo) < 0.0) { ns = -ns; }
        let eps = offset_for(point);
        let above = point + ng * eps;
        var floor_alpha = 0.0;
        if (scattered) { floor_alpha = params.misc.x; }

        // Delta bounces: mirrors, glass, smooth coats.
        var delta = false;
        var next_dir = vec3f(0.0);
        var next_from = above;
        var attenuate = vec3f(1.0);
        var lobe = make_lobe(0u, ns);
        var base = vec3f(0.0);
        if (mat.kind == 0u) {
            lobe.albedo = mat.c0.xyz;
        } else if (mat.kind == 1u) {
            let alpha = max(mat.p.y * mat.p.y, floor_alpha);
            if (alpha >= MIN_ALPHA) {
                lobe.kind = 2u; lobe.fresnel = 1u; lobe.alpha = alpha; lobe.ior = mat.p.x; base = mat.c0.xyz;
            } else {
                let coat = fresnel_reflectance(abs(dot(wo, ns)), 1.0 / mat.p.x, mat.p.x);
                if (rand() < coat) {
                    delta = true; next_dir = reflect_about(rd, ns);
                } else {
                    lobe.albedo = mat.c0.xyz;
                }
            }
        } else if (mat.kind == 3u || mat.kind == 5u) {
            let alpha = max(mat.p.y * mat.p.y, floor_alpha);
            lobe.kind = 1u; lobe.alpha = alpha; lobe.albedo = mat.c0.xyz; lobe.k = mat.c1.xyz;
            lobe.fresnel = select(2u, 0u, mat.kind == 5u);
            if (alpha < MIN_ALPHA) {
                delta = true; next_dir = reflect_about(rd, ns);
                attenuate = fresnel_eval(lobe, abs(dot(wo, ns)));
            }
        } else if (mat.kind == 2u) {
            // As `functions::scatter_dielectric`.
            let n = hit.ns;
            let exiting = dot(rd, n) > 0.0;
            let outward = select(n, -n, exiting);
            let ior = mat.p.x;
            let ni_over_nt = select(1.0 / ior, ior, exiting);
            let r = fresnel_reflectance(abs(dot(rd, n)), ni_over_nt, ior);
            let dt = dot(rd, outward);
            let disc = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
            delta = true;
            if (disc > 0.0 && rand() >= r) {
                next_dir = normalize((rd - outward * dt) * ni_over_nt - outward * sqrt(disc));
                next_from = point - outward * eps;
                attenuate = mat.c0.xyz;
            } else {
                next_dir = reflect_about(rd, n);
                next_from = point + outward * eps;
            }
        }

        if (delta) {
            if (depth >= max_depth) { break; }
            // Russian roulette, as `trace::continuation_weight`.
            if (length(attenuate) <= params.misc.y) { break; }
            if (params.seed.y == 1u && f32(depth) >= params.misc.w) {
                let q = min(max(max(attenuate.x, attenuate.y), attenuate.z), 1.0);
                if (rand() >= q) { break; }
                beta /= q;
            }
            beta *= attenuate;
            specular = true;
            ro = next_from;
            rd = normalize(next_dir);
            depth += 1u;
            continue;
        }

        scattered = true;
        // Point lights, every one.
        for (var li = 0u; li < params.counts.w; li++) {
            let light = lights[li];
            let to = light.pos.xyz - above;
            let d2 = dot(to, to);
            let wi = to / sqrt(d2);
            let f = bsdf_eval(lobe, base, wo, wi);
            if (dot(f, f) > 0.0) {
                let i = light.intensity.xyz * spot(light, -wi);
                if (dot(i, i) > 0.0 && !occluded(above, wi, sqrt(d2))) {
                    radiance += beta * f * i / d2;
                }
            }
        }
        // The sun: a point for lobes that can't resolve its disc.
        if ((flags & FLAG_SUN) != 0u) {
            let sd = params.sun_dir.xyz;
            let solid_angle = 2.0 * PI * (1.0 - params.sun_dir.w);
            if (bsdf_pdf(lobe, wo, sd) * solid_angle < 1e-3) {
                let f = bsdf_eval(lobe, base, wo, sd);
                if (dot(f, f) > 0.0 && !occluded(above, sd, 1e30)) {
                    radiance += beta * f * params.sun_irr.xyz;
                }
            } else {
                // Uniformly over the disc's cone, weighted by its radiance.
                let cos_t = 1.0 - rand() * (1.0 - params.sun_dir.w);
                let sin_t = sqrt(max(1.0 - cos_t * cos_t, 0.0));
                let phi = 2.0 * PI * rand();
                let tb = basis(sd);
                let wi = tb[0] * (sin_t * cos(phi)) + tb[1] * (sin_t * sin(phi)) + sd * cos_t;
                let pdf = sun_disc_pdf(wi);
                let f = bsdf_eval(lobe, base, wo, wi);
                if (pdf > 0.0 && dot(f, f) > 0.0 && !occluded(above, wi, 1e30)) {
                    let w = power_heuristic(pdf, bsdf_pdf(lobe, wo, wi));
                    radiance += beta * f * sun_disc(wi) * (w / pdf);
                }
            }
        }
        // One area light, by power.
        if ((flags & FLAG_SAMPLE_LIGHTS) != 0u && n_emitters > 0u) {
            let e = search(params.offs.y, n_emitters, rand());
            let pmf = emitter_probability(e);
            let pi = table_u32(params.offs.x + e);
            let ls = sample_light(pi, above, rand(), rand());
            let to = ls.y - above;
            let d = length(to);
            let wi = to / d;
            let le = emitted(prims[pi].info.y, ls.n, -wi);
            if (ls.pdf > 0.0 && dot(le, le) > 0.0) {
                let lp = pmf * ls.pdf;
                let f = bsdf_eval(lobe, base, wo, wi);
                if (dot(f, f) > 0.0 && !occluded(above, wi, d * (1.0 - 1e-4) - eps)) {
                    let w = power_heuristic(lp, bsdf_pdf(lobe, wo, wi));
                    radiance += beta * f * le * (w / lp);
                }
            }
        }
        // The sky, by its brightness.
        if ((flags & FLAG_SAMPLE_ENV) != 0u) {
            let es = env_sample(rand(), rand());
            if (es.w > 0.0) {
                let f = bsdf_eval(lobe, base, wo, es.xyz);
                if (dot(f, f) > 0.0 && !occluded(above, es.xyz, 1e30)) {
                    let w = power_heuristic(es.w, bsdf_pdf(lobe, wo, es.xyz));
                    radiance += beta * f * env_lookup(es.xyz) * (w / es.w);
                }
            }
        }
        // The next ray.
        if (depth >= max_depth) { break; }
        let s = bsdf_sample(lobe, wo, rand(), rand(), rand());
        if (s.w <= 0.0) { break; }
        let weight = bsdf_eval(lobe, base, wo, s.xyz) / s.w;
        if (length(weight) <= params.misc.y) { break; }
        if (params.seed.y == 1u && f32(depth) >= params.misc.w) {
            let q = min(max(max(weight.x, weight.y), weight.z), 1.0);
            if (rand() >= q) { break; }
            beta /= q;
        }
        beta *= weight;
        prev_pdf = s.w;
        specular = false;
        var side = ng;
        if (dot(s.xyz, ng) < 0.0) { side = -ng; }
        ro = point + side * eps;
        rd = s.xyz;
        depth += 1u;
    }
    let c = radiance * cam.o.w;
    if (all(c == c)) {
        accum[index] += vec4f(c, 1.0);
    }
}
