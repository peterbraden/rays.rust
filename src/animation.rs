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
}

impl Default for Animation {
    fn default() -> Animation {
        Animation { fps: 24., start: 0., frames: None }
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
    fn test_frame_paths() {
        assert_eq!(frame_path("out/%04d.png", 7), "out/0007.png");
        assert_eq!(frame_path("f%d.exr", 123), "f123.exr");
        assert_eq!(frame_path("still.png", 3), "still.png");
    }
}
