//! # Animation: the scene's clock
//!
//! A frame of an animation is a pure function of the scene file, its seed and the frame's time,
//! so frames can be rendered alone, in any order, on any machine (see docs/animation.md).
//!
//! **Video time** `t` is seconds of the finished film. The scene's `"animation"` block says how
//! frames map onto it: frame `n` is at `t = start + n / fps`.
//!
//! ```json
//! "animation": { "fps": 24, "frames": 240, "start": 0 }
//! ```
//!
//! **Clocks.** Each thing that changes with time (`TIMED`) reads its own clock, which runs at a
//! rate with units: `"1 s/s"` (real time, the default), `"1 min/s"` (a time-lapse), `"2 kyr/s"`
//! (geology), `"0"` (frozen). A bare number is a multiple of real time. Clocks are shared by
//! name or given inline:
//!
//! ```json
//! "clocks": { "geology": "2 kyr/s", "weather": "1 min/s" },
//! "objects": [
//!   {"type": "landscape", "time": 1e6, "clock": "geology"},
//!   {"type": "harris_cumulus", "time": 1800, "clock": "weather"},
//!   {"type": "ocean"}
//! ]
//! ```
//!
//! An object's `"time"` (in its own unit: seconds, or years for a landscape) is where it is at
//! video time 0, which is also what a still shows; at video time `t` it is at `time + rate t`,
//! converted to its unit. Frame 0 is at video time `start`, which may be negative: an animation
//! can lead up to the still. `apply_clocks` resolves this before the scene is parsed, leaving each timed
//! object the time that has elapsed on its clock in `"clock_elapsed"`, which its parser adds to
//! its `"time"` (`SceneFile::local_time`), so each keeps its own default time.
//!
//! **Keyed values.** Any number or list of numbers in the scene can be given as keys in video
//! seconds, `{"keys": [[0, [0, 2, 0]], [10, [0, 4e5, 0]]], "interpolation": "smooth"}`, and is
//! resolved to its value at the frame's time before the scene is parsed (`resolve_keys`). A
//! keyed camera (`location`, `lookat`, `up`) follows its path, and blurs along it while the
//! shutter is open (`"animation": {"shutter": 0.5}`, in frames: its pose at the shutter's close
//! becomes its `"motion"`).
use serde_json::{Map, Value};

/// A year, in seconds (a Julian year).
pub const YEAR: f64 = 365.25 * 86400.;

/// The key in which `apply_clocks` leaves the time elapsed on an object's clock, in its unit.
pub const ELAPSED: &str = "clock_elapsed";

/// The objects that change with time, and the length of their unit of time in seconds.
pub const TIMED: &[(&str, f64)] = &[
    ("ocean", 1.),
    ("firework", 1.),
    ("hogan_kew_cirrus", 1.),
    ("harris_cumulus", 1.),
    ("lenticular", 1.),
    ("landscape", YEAR),
];

/// The length of an object type's unit of time in seconds, if it changes with time.
pub fn unit_of(object_type: &str) -> Option<f64> {
    TIMED.iter().find(|(t, _)| *t == object_type).map(|(_, u)| *u)
}

/// The length of a unit of time, in seconds.
fn unit_seconds(unit: &str) -> Option<f64> {
    Some(match unit {
        "s" | "sec" | "second" | "seconds" => 1.,
        "min" | "minute" | "minutes" => 60.,
        "h" | "hr" | "hour" | "hours" => 3600.,
        "day" | "days" => 86400.,
        "yr" | "year" | "years" => YEAR,
        "kyr" => 1e3 * YEAR,
        "Myr" => 1e6 * YEAR,
        _ => return None,
    })
}

/// A clock's rate: seconds of its time per second of video. `"2 kyr/s"`, `"1 min/s"`, `"0"`, or
/// a number (a multiple of real time).
pub fn parse_rate(v: &Value) -> Result<f64, String> {
    let bad = || format!("a clock's rate is a number or eg. \"1 s/s\", \"2 kyr/s\", got {}", v);
    match v {
        Value::Number(n) => n.as_f64().ok_or_else(bad),
        Value::String(s) => {
            let s = s.trim();
            if let Ok(x) = s.parse::<f64>() {
                return Ok(x);
            }
            let (amount, per) = s.split_once('/').ok_or_else(bad)?;
            let mut words = amount.split_whitespace();
            let (x, unit) = match (words.next(), words.next(), words.next()) {
                (Some(x), Some(u), None) => (x.parse::<f64>().map_err(|_| bad())?, u),
                _ => return Err(bad()),
            };
            let per = unit_seconds(per.trim()).ok_or_else(bad)?;
            Ok(x * unit_seconds(unit).ok_or_else(bad)? / per)
        },
        _ => Err(bad()),
    }
}

/// The scene's `"animation"`: how frames map onto video time.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Animation {
    /// Frames per second of video.
    pub fps: f64,
    /// Video time of frame 0, seconds.
    pub start: f64,
    /// How many frames there are, if said.
    pub frames: Option<u64>,
    /// How long the shutter is open, as a fraction of a frame (0.5: a 180° shutter); keyed
    /// cameras blur along their paths over it.
    pub shutter: f64,
}

impl Default for Animation {
    fn default() -> Animation {
        Animation { fps: 24., start: 0., frames: None, shutter: 0.5 }
    }
}

impl Animation {
    pub fn parse(v: &Value) -> Result<Animation, String> {
        let mut a = Animation::default();
        match v {
            Value::Null => {},
            Value::Object(o) => {
                let num = |k: &str| o.get(k).map(|x| x.as_f64().ok_or_else(|| format!("\"animation\": \"{}\" must be a number, got {}", k, x)));
                if let Some(f) = num("fps") { a.fps = f?; }
                if let Some(s) = num("start") { a.start = s?; }
                if let Some(s) = num("shutter") { a.shutter = s?.max(0.); }
                if let Some(n) = o.get("frames") {
                    a.frames = Some(n.as_u64().ok_or_else(|| format!("\"animation\": \"frames\" must be a whole number, got {}", n))?);
                }
                if !(a.fps > 0. && a.fps.is_finite()) {
                    return Err(format!("\"animation\": \"fps\" must be positive, got {}", a.fps));
                }
            },
            _ => return Err(format!("\"animation\" must be an object, got {}", v)),
        }
        Ok(a)
    }

    /// The video time of a moment.
    pub fn time(&self, when: When) -> f64 {
        match when {
            When::Frame(n) => self.start + n as f64 / self.fps,
            When::Time(t) => t,
        }
    }
}

/// A moment of an animation: a frame, or any video time (seconds).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum When {
    Frame(u64),
    Time(f64),
}

/// Resolve the clocks of a scene file at video time `elapsed` (seconds after time 0): removes
/// `"clocks"` and each object's `"clock"`, and gives each timed object the time elapsed on its
/// clock, in its unit, as `ELAPSED` (only when time has passed, so a still is unchanged).
pub fn apply_clocks(doc: &mut Value, elapsed: f64) -> Result<(), String> {
    let clocks = match doc.as_object_mut().and_then(|o| o.remove("clocks")) {
        None | Some(Value::Null) => Map::new(),
        Some(Value::Object(m)) => m,
        Some(v) => return Err(format!("\"clocks\" must be an object of names and rates, got {}", v)),
    };
    let mut rates = std::collections::HashMap::new();
    for (name, v) in &clocks {
        rates.insert(name.clone(), parse_rate(v).map_err(|e| format!("clock \"{}\": {}", name, e))?);
    }
    resolve(doc, elapsed, &rates)
}

fn resolve(v: &mut Value, elapsed: f64, rates: &std::collections::HashMap<String, f64>) -> Result<(), String> {
    match v {
        Value::Object(o) => {
            let object_type = o.get("type").and_then(Value::as_str).map(str::to_string);
            let unit = object_type.as_deref().and_then(unit_of);
            let clock = o.remove("clock");
            match (unit, clock) {
                (Some(unit), clock) => {
                    let rate = match &clock {
                        None => 1.,
                        Some(Value::String(name)) if rates.contains_key(name) => rates[name],
                        Some(Value::String(s)) if parse_rate(&Value::String(s.clone())).is_err() =>
                            return Err(format!("unknown clock \"{}\" (the scene's \"clocks\" are: {})", s,
                                rates.keys().cloned().collect::<Vec<_>>().join(", "))),
                        Some(c) => parse_rate(c)?,
                    };
                    if elapsed != 0. {
                        o.insert(ELAPSED.to_string(), Value::from(rate * elapsed / unit));
                    }
                },
                (None, Some(_)) => return Err(format!("\"clock\" on {}, which doesn't change with time (only {} do)",
                    object_type.map_or("an object without a type".to_string(), |t| format!("a \"{}\"", t)),
                    TIMED.iter().map(|(t, _)| *t).collect::<Vec<_>>().join(", "))),
                (None, None) => {},
            }
            for (_, child) in o.iter_mut() {
                resolve(child, elapsed, rates)?;
            }
        },
        Value::Array(items) => for i in items { resolve(i, elapsed, rates)? },
        _ => {},
    }
    Ok(())
}

/// Is `v` a keyed value: `{"keys": [[t, value], ...], "interpolation": ...}`?
fn is_keyed(v: &Value) -> bool {
    v.as_object().is_some_and(|o| o.contains_key("keys") && o.keys().all(|k| k == "keys" || k == "interpolation"))
}

/// A keyed value at video time `t`. Keys are `[time, value]` in video seconds, the values
/// numbers or arrays of numbers (vectors, colours), held at the first and last outside their
/// times. `"interpolation"`: `"smooth"` (the default: a monotone cubic through the keys, which
/// never overshoots them, so a camera climbing from 2 m never dips below it), `"linear"`, or
/// `"step"` (each value held until the next key; any values).
pub fn keyed_value(v: &Value, t: f64) -> Result<Value, String> {
    let keys = v["keys"].as_array().filter(|k| !k.is_empty())
        .ok_or_else(|| format!("\"keys\" must be a list of [time, value], got {}", v["keys"]))?;
    let mut times = Vec::with_capacity(keys.len());
    let mut values = Vec::with_capacity(keys.len());
    for k in keys {
        match k.as_array().map(|a| a.as_slice()) {
            Some([time, value]) => {
                let time = time.as_f64().ok_or_else(|| format!("a key's time must be a number, got {}", k))?;
                if times.last().is_some_and(|&last| time <= last) {
                    return Err(format!("keys must be in order of time, got {} after {}", time, times.last().unwrap()));
                }
                times.push(time);
                values.push(value.clone());
            },
            _ => return Err(format!("a key must be [time, value], got {}", k)),
        }
    }
    let interpolation = v["interpolation"].as_str().unwrap_or("smooth");
    if !["smooth", "linear", "step"].contains(&interpolation) {
        return Err(format!("unknown \"interpolation\" {}: expected \"smooth\", \"linear\" or \"step\"", v["interpolation"]));
    }
    // The segment: the last key at or before t.
    let i = times.iter().rposition(|&k| k <= t);
    if interpolation == "step" {
        return Ok(values[i.unwrap_or(0)].clone());
    }
    // Each value as numbers.
    let numbers = |x: &Value| -> Result<Vec<f64>, String> {
        match x {
            Value::Number(n) => Ok(vec![n.as_f64().unwrap()]),
            Value::Array(a) => a.iter().map(|c| c.as_f64().ok_or_else(|| format!("keyed values must be numbers or lists of numbers, got {}", x))).collect(),
            _ => Err(format!("keyed values must be numbers or lists of numbers (or \"interpolation\": \"step\"), got {}", x)),
        }
    };
    let ys = values.iter().map(numbers).collect::<Result<Vec<_>, _>>()?;
    if ys.iter().any(|y| y.len() != ys[0].len()) {
        return Err(format!("keyed values must all have the same length, got {}", v["keys"]));
    }
    let n = times.len();
    let out: Vec<f64> = match i {
        None => ys[0].clone(),
        Some(i) if i == n - 1 => ys[n - 1].clone(),
        Some(i) => (0..ys[0].len()).map(|c| {
            let y: Vec<f64> = ys.iter().map(|y| y[c]).collect();
            match interpolation {
                "linear" => y[i] + (y[i + 1] - y[i]) * (t - times[i]) / (times[i + 1] - times[i]),
                _ => monotone_cubic(&times, &y, i, t),
            }
        }).collect(),
    };
    Ok(match &values[0] {
        Value::Number(_) => Value::from(out[0]),
        _ => Value::from(out),
    })
}

/// A monotone cubic (Fritsch and Carlson, "Monotone Piecewise Cubic Interpolation", SIAM J.
/// Numer. Anal. 17(2), 1980), with Fritsch and Butland's weighted harmonic mean for the slopes,
/// through (`x`, `y`), in segment `i`, at `t`. Between keys it stays between their values.
fn monotone_cubic(x: &[f64], y: &[f64], i: usize, t: f64) -> f64 {
    let n = x.len();
    let delta = |k: usize| (y[k + 1] - y[k]) / (x[k + 1] - x[k]);
    let slope = |k: usize| -> f64 {
        if k == 0 { return delta(0); }
        if k == n - 1 { return delta(n - 2); }
        let (d0, d1) = (delta(k - 1), delta(k));
        if d0 * d1 <= 0. { return 0.; }
        let (h0, h1) = (x[k] - x[k - 1], x[k + 1] - x[k]);
        3. * (h0 + h1) / ((2. * h1 + h0) / d0 + (h1 + 2. * h0) / d1)
    };
    // The end slopes, limited so the end segments don't overshoot either.
    let limit = |m: f64, d: f64| if m * d <= 0. { 0. } else if m.abs() > 3. * d.abs() { 3. * d } else { m };
    let h = x[i + 1] - x[i];
    let d = delta(i);
    let (m0, m1) = (limit(slope(i), d), limit(slope(i + 1), d));
    let s = (t - x[i]) / h;
    let (s2, s3) = (s * s, s * s * s);
    (2. * s3 - 3. * s2 + 1.) * y[i] + (s3 - 2. * s2 + s) * h * m0 + (-2. * s3 + 3. * s2) * y[i + 1] + (s3 - s2) * h * m1
}

/// Resolve every keyed value in a scene file at video time `t`. A camera whose `location`,
/// `lookat` or `up` is keyed also gets a `"motion"` to its pose at `close` (when the shutter
/// closes, if it is open for a while), so it blurs along its path.
pub fn resolve_keys(doc: &mut Value, t: f64, close: Option<f64>) -> Result<(), String> {
    if let (Some(close), Some(camera)) = (close, doc.get_mut("camera").and_then(Value::as_object_mut)) {
        let moving: Vec<&str> = ["location", "lookat", "up"].into_iter().filter(|k| camera.get(*k).is_some_and(is_keyed)).collect();
        if !moving.is_empty() && !camera.contains_key("motion") {
            let mut end = Map::new();
            for k in moving {
                end.insert(k.to_string(), keyed_value(&camera[k], close).map_err(|e| format!("camera \"{}\": {}", k, e))?);
            }
            camera.insert("motion".to_string(), Value::Object(end));
        }
    }
    resolve_keyed(doc, t)
}

fn resolve_keyed(v: &mut Value, t: f64) -> Result<(), String> {
    if is_keyed(v) {
        *v = keyed_value(v, t)?;
        return Ok(());
    }
    match v {
        Value::Object(o) => for (k, child) in o.iter_mut() {
            resolve_keyed(child, t).map_err(|e| if e.starts_with('"') { e } else { format!("\"{}\": {}", k, e) })?;
        },
        Value::Array(items) => for i in items { resolve_keyed(i, t)? },
        _ => {},
    }
    Ok(())
}

/// Substitute a frame number into an output path: `%d`, or `%0Nd` for N digits (eg.
/// `frames/%04d.png`). A path without one is returned unchanged.
pub fn frame_path(pattern: &str, frame: u64) -> String {
    if let Some(i) = pattern.find('%') {
        let rest = &pattern[i + 1 ..];
        if let Some(d) = rest.find('d') {
            let spec = &rest[.. d];
            if spec.is_empty() || spec.chars().all(|c| c.is_ascii_digit()) {
                let width = spec.parse::<usize>().unwrap_or(0);
                return format!("{}{:0width$}{}", &pattern[.. i], frame, &rest[d + 1 ..], width = width);
            }
        }
    }
    pattern.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_rates_have_units() {
        assert_eq!(parse_rate(&json!("1 s/s")).unwrap(), 1.);
        assert_eq!(parse_rate(&json!("1 min/s")).unwrap(), 60.);
        assert_eq!(parse_rate(&json!("2 kyr/s")).unwrap(), 2000. * YEAR);
        assert_eq!(parse_rate(&json!("1 h / day")).unwrap(), 1. / 24.);
        assert_eq!(parse_rate(&json!("0")).unwrap(), 0.);
        assert_eq!(parse_rate(&json!(60)).unwrap(), 60.);
        assert!(parse_rate(&json!("2 kyr")).is_err());
        assert!(parse_rate(&json!("fast")).is_err());
    }

    #[test]
    fn test_frames_map_to_video_time() {
        let a = Animation::parse(&json!({ "fps": 25, "start": 2 })).unwrap();
        assert_eq!(a.time(When::Frame(0)), 2.);
        assert_eq!(a.time(When::Frame(50)), 4.);
        assert_eq!(a.time(When::Time(7.5)), 7.5);
        assert!(Animation::parse(&json!({ "fps": 0 })).is_err());
    }

    #[test]
    fn test_each_object_keeps_its_own_clock() {
        let mut doc = json!({
            "clocks": { "geology": "2 kyr/s", "weather": "1 min/s" },
            "objects": [
                { "type": "landscape", "time": 1e6, "clock": "geology" },
                { "type": "harris_cumulus", "clock": "weather" },
                { "type": "ocean" },
                { "type": "lenticular", "clock": "10 s/s" },
                { "type": "sphere" },
            ]
        });
        apply_clocks(&mut doc, 10.).unwrap();
        let o = &doc["objects"];
        assert!(doc.get("clocks").is_none() && o[0].get("clock").is_none());
        assert!((o[0][ELAPSED].as_f64().unwrap() - 20_000.).abs() < 1e-6, "{}", o[0]);
        assert_eq!(o[1][ELAPSED], json!(600.));
        assert_eq!(o[2][ELAPSED], json!(10.));
        assert_eq!(o[3][ELAPSED], json!(100.));
        assert!(o[4].get(ELAPSED).is_none());
        // The landscape's own time is untouched: its parser adds the two.
        assert_eq!(o[0]["time"], json!(1e6));
    }

    #[test]
    fn test_a_still_is_unchanged() {
        let mut doc = json!({ "objects": [{ "type": "ocean", "time": 3, "clock": "1 min/s" }] });
        apply_clocks(&mut doc, 0.).unwrap();
        assert_eq!(doc, json!({ "objects": [{ "type": "ocean", "time": 3 }] }));
    }

    #[test]
    fn test_clock_errors() {
        let err = |doc: Value| { let mut d = doc; apply_clocks(&mut d, 1.).unwrap_err() };
        assert!(err(json!({ "objects": [{ "type": "ocean", "clock": "tides" }] })).contains("unknown clock"));
        assert!(err(json!({ "objects": [{ "type": "sphere", "clock": "1 s/s" }] })).contains("doesn't change with time"));
        assert!(err(json!({ "clocks": { "x": "quick" } })).contains("clock \"x\""));
    }

    #[test]
    fn test_keyed_values() {
        let v = |keys: Value, t: f64| keyed_value(&keys, t).unwrap();
        let k = json!({ "keys": [[0, 2], [1, 20], [2, 200], [3, 2000]] });
        // Held outside the keys, through them at their times.
        assert_eq!(v(k.clone(), -1.), json!(2.));
        assert_eq!(v(k.clone(), 1.), json!(20.));
        assert_eq!(v(k.clone(), 9.), json!(2000.));
        // Smooth, but never below the first key on the way up (a Catmull-Rom would dip to 0.9).
        for i in 0..=300 {
            let t = i as f64 / 100.;
            let (a, b) = (v(k.clone(), t).as_f64().unwrap(), v(k.clone(), t + 0.01).as_f64().unwrap());
            assert!(a >= 2. && b >= a, "at {}: {} then {}", t, a, b);
        }
        // Vectors, linearly; and steps of anything.
        assert_eq!(v(json!({ "keys": [[0, [0, 0, 0]], [2, [2, 4, 6]]], "interpolation": "linear" }), 0.5), json!([0.5, 1., 1.5]));
        assert_eq!(v(json!({ "keys": [[0, "a"], [1, "b"]], "interpolation": "step" }), 0.5), json!("a"));
        assert!(keyed_value(&json!({ "keys": [[1, 0], [0, 1]] }), 0.).is_err());
        assert!(keyed_value(&json!({ "keys": [[0, 0], [1, 1]], "interpolation": "cubic" }), 0.).is_err());
    }

    #[test]
    fn test_a_keyed_camera_moves_while_the_shutter_is_open() {
        let mut doc = json!({
            "camera": { "location": { "keys": [[0, [0, 0, 0]], [1, [24, 0, 0]]], "interpolation": "linear" },
                        "lookat": [0, 0, 1], "up": [0, 1, 0] },
            "objects": [{ "type": "sphere", "radius": { "keys": [[0, 1], [1, 3]], "interpolation": "linear" } }]
        });
        resolve_keys(&mut doc, 0.5, Some(0.5 + 0.5 / 24.)).unwrap();
        assert_eq!(doc["camera"]["location"], json!([12., 0., 0.]));
        assert_eq!(doc["camera"]["motion"], json!({ "location": [12.5, 0., 0.] }));
        assert_eq!(doc["objects"][0]["radius"], json!(2.));
    }

    #[test]
    fn test_frame_paths() {
        assert_eq!(frame_path("out/%04d.png", 7), "out/0007.png");
        assert_eq!(frame_path("f%d.exr", 123), "f123.exr");
        assert_eq!(frame_path("still.png", 3), "still.png");
    }
}
