//! A render's report: what each run that added samples to it did, kept with its samples in
//! the saved state (`<output>.accum`, `RenderContext::encode_state`), so that how every sample
//! was made can be looked into after the render, rather than read off a terminal at the time.
//!
//! A run is one invocation of the renderer on a render: a fresh one, or one resumed from the
//! saved state, which keeps the runs before it and adds its own (plan.md, section 16). Fields
//! are added to `Run` as the renderer records more; each takes a default, so a report written
//! before a field existed still reads.

/// The runs that made a render's samples, oldest first.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Report {
    pub runs: Vec<Run>,
}

/// One run of the renderer on a render.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Run {
    /// When it started, in seconds since 1970 (UTC).
    pub started: u64,
    /// How long it had run when its state was last saved, in seconds.
    pub seconds: f64,
    /// The render's samples, summed over its pixels, when the run began (those resumed) and
    /// when its state was last saved.
    pub samples_before: u64,
    pub samples_after: u64,
}

impl Run {
    /// A run beginning now.
    pub fn starting() -> Run {
        let started = web_time::SystemTime::now().duration_since(web_time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
        Run { started, ..Run::default() }
    }
}

impl Report {
    /// As bytes for the saved state: JSON.
    pub fn encode(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("a report is plain data")
    }

    /// From the saved state's bytes.
    pub fn decode(bytes: &[u8]) -> Result<Report, String> {
        serde_json::from_slice(bytes).map_err(|e| format!("its report is unreadable ({})", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_report_reads_back_as_written() {
        let report = Report { runs: vec![
            Run { started: 1_795_000_000, seconds: 12.5, samples_before: 0, samples_after: 384 },
            Run { started: 1_795_000_100, seconds: 3.25, samples_before: 384, samples_after: 768 },
        ] };
        assert_eq!(Report::decode(&report.encode()), Ok(report));
    }

    #[test]
    fn test_a_run_missing_fields_takes_their_defaults() {
        // A report written before a field existed.
        let old = br#"{"runs": [{"started": 5, "seconds": 1.5}]}"#;
        let report = Report::decode(old).unwrap();
        assert_eq!(report.runs, vec![Run { started: 5, seconds: 1.5, ..Run::default() }]);
    }
}
