//! A profile of the render: the time each pixel took (`--time-map`), as a heatmap for viewing
//! or as raw seconds for analysis.
//!
//! Each pixel's time is measured around its samples (`rendercontext::render_pixel`) by the
//! rendering thread's CPU clock (`thread_time`), and summed over its passes: so it is the cost
//! of the pixel's light transport, including the extra samples adaptive sampling gave it, but
//! not time the thread spent descheduled, which a wall clock would scatter over the map as
//! streaks. Only this run's samples are timed; a resumed render's map covers what was rendered
//! since it resumed.

use crate::color::Color;
use crate::output::LinearImage;

/// The fraction of pixels below the heatmap's brightest colour: the slowest few are clamped
/// to it, so one pathological pixel doesn't leave the rest of the map black.
const PERCENTILE: f64 = 0.995;

/// The CPU time the calling thread has used, in seconds: differences of it time a pixel.
/// (The command line's: `libc` comes with the `cli` feature, and a browser has no such clock.)
#[cfg(all(unix, feature = "cli"))]
pub fn thread_time() -> f64 {
    let mut ts: libc::timespec = unsafe { std::mem::zeroed() };
    if unsafe { libc::clock_gettime(libc::CLOCK_THREAD_CPUTIME_ID, &mut ts) } != 0 {
        return wall_time();
    }
    ts.tv_sec as f64 + ts.tv_nsec as f64 * 1e-9
}

/// Without a per-thread CPU clock, the wall clock.
#[cfg(not(all(unix, feature = "cli")))]
pub fn thread_time() -> f64 {
    wall_time()
}

/// Seconds since the first call.
fn wall_time() -> f64 {
    static START: std::sync::OnceLock<web_time::Instant> = std::sync::OnceLock::new();
    START.get_or_init(web_time::Instant::now).elapsed().as_secs_f64()
}

/// A summary of a render's times per pixel, in seconds.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Summary {
    pub total: f64,
    pub mean: f64,
    pub max: f64,
    /// The time the heatmap's brightest colour stands for (`PERCENTILE` of the pixels).
    pub scale: f64,
}

/// Summarise per-pixel times.
pub fn summary(times: &[f64]) -> Summary {
    let total: f64 = times.iter().sum();
    let mut sorted: Vec<f64> = times.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let max = sorted.last().copied().unwrap_or(0.);
    let scale = if sorted.is_empty() {
        0.
    } else {
        sorted[(((sorted.len() - 1) as f64) * PERCENTILE).round() as usize]
    };
    Summary { total, mean: total / times.len().max(1) as f64, max, scale: if scale > 0. { scale } else { max } }
}

/// The map of per-pixel `times` (row by row, bottom row first, as `RenderContext` holds them),
/// top row first. Raw, each pixel is its time in seconds (for an EXR); otherwise its time
/// relative to the summary's `scale`, coloured by `heat` (for viewing: write it without a
/// transfer function, the colours are already display-encoded).
pub fn image(times: &[f64], width: usize, height: usize, raw: bool) -> LinearImage {
    let scale = summary(times).scale;
    let pixels = (0..height).rev().flat_map(|y| (0..width).map(move |x| y * width + x))
        .map(|i| {
            let t = times[i];
            if raw {
                Color::new(t, t, t)
            } else {
                heat(if scale > 0. { t / scale } else { 0. })
            }
        }).collect();
    LinearImage { width, height, pixels }
}

/// The "inferno" colour map (black through purple and orange to pale yellow; perceptually
/// uniform, and legible in greyscale), at `t` in [0, 1]: a polynomial fit to it (M. Zucker,
/// after matplotlib's), display-encoded.
pub fn heat(t: f64) -> Color {
    const C: [[f64; 3]; 7] = [
        [0.000_218_940_369_119_226_5, 0.001_651_004_631_001_012, -0.019_480_898_437_091_84],
        [0.106_513_419_485_611_6, 0.563_956_436_788_409_1, 3.932_712_388_889_277],
        [11.602_493_082_471_87, -3.972_853_965_665_698, -15.942_394_106_291_4],
        [-41.703_996_131_394_59, 17.436_398_882_053_13, 44.354_145_198_728_13],
        [77.162_935_699_427, -33.402_358_942_100_92, -81.807_309_257_389_93],
        [-71.319_428_244_992_14, 32.626_064_263_977_23, 73.209_519_858_032_02],
        [25.131_126_224_773_41, -12.242_668_952_385_67, -23.070_325_002_871_72],
    ];
    let t = if t.is_nan() { 0. } else { t.clamp(0., 1.) };
    let channel = |k: usize| C.iter().rev().fold(0., |acc, c| acc * t + c[k]).clamp(0., 1.);
    Color::new(channel(0), channel(1), channel(2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heat_runs_dark_to_bright() {
        let lum = |c: Color| 0.2126 * c.rgb.x + 0.7152 * c.rgb.y + 0.0722 * c.rgb.z;
        let samples: Vec<f64> = (0..=10).map(|i| lum(heat(i as f64 / 10.))).collect();
        assert!(samples[0] < 0.02, "starts black: {:?}", samples);
        assert!(samples[10] > 0.9, "ends pale: {:?}", samples);
        assert!(samples.windows(2).all(|w| w[1] > w[0]), "brightens throughout: {:?}", samples);
        // Out of range and NaN are clamped.
        assert_eq!(heat(-1.), heat(0.));
        assert_eq!(heat(2.), heat(1.));
        assert_eq!(heat(f64::NAN), heat(0.));
    }

    #[test]
    fn thread_time_advances_with_work() {
        let start = thread_time();
        let mut x = 0u64;
        for i in 0..20_000_000u64 {
            x = std::hint::black_box(x.wrapping_mul(6364136223846793005).wrapping_add(i));
        }
        std::hint::black_box(x);
        assert!(thread_time() > start);
    }

    #[test]
    fn summary_clamps_outliers() {
        let mut times = vec![1.; 1000];
        times[3] = 1000.;
        let s = summary(&times);
        assert_eq!(s.max, 1000.);
        assert_eq!(s.scale, 1.);
        assert!((s.total - 1999.).abs() < 1e-9);
        assert!((s.mean - 1.999).abs() < 1e-9);
    }

    #[test]
    fn image_is_top_row_first() {
        // Bottom row slow, top row fast.
        let times = [2., 2., 0., 0.];
        let raw = image(&times, 2, 2, true);
        assert_eq!(raw.get(0, 0), Color::new(0., 0., 0.));
        assert_eq!(raw.get(1, 1), Color::new(2., 2., 2.));
        let map = image(&times, 2, 2, false);
        assert_eq!(map.get(0, 1), heat(1.));
        assert_eq!(map.get(0, 0), heat(0.));
    }
}
