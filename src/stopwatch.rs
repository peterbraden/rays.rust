//! Elapsed time, for reporting how long things took to build.
//!
//! `std::time::Instant` panics on `wasm32-unknown-unknown`, which has no clock; there a
//! `Stopwatch` always reads zero, so code that times itself still runs in a browser.
use std::time::Duration;

pub struct Stopwatch {
    #[cfg(not(target_arch = "wasm32"))]
    start: std::time::Instant,
}

impl Stopwatch {
    #[allow(clippy::new_without_default)]
    pub fn start() -> Stopwatch {
        Stopwatch {
            #[cfg(not(target_arch = "wasm32"))]
            start: std::time::Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        #[cfg(not(target_arch = "wasm32"))]
        return self.start.elapsed();
        #[cfg(target_arch = "wasm32")]
        return Duration::ZERO;
    }
}
