//! Journal telemetry for RelayRing.

use crate::wire::{decode, validate};
use crate::journal::ring_log::RingLog;
use std::fmt;

#[derive(Debug, Clone)]
pub struct JournalReport {
    pub record_count: usize,
    pub ring_tail: u64,
    pub violations: usize,
    pub digest: u64,
}

impl fmt::Display for JournalReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "records={} tail={} violations={} digest={:#x}", self.record_count, self.ring_tail, self.violations, self.digest)
    }
}

pub fn run(data: &[u8]) -> Result<JournalReport, String> {
    let frame = decode::decode(data)?;
    let issues = validate::validate_frame(data, true)?;
    let tail = RingLog::with_capacity(32).tail;
    Ok(JournalReport {
        record_count: frame.section_count(),
        ring_tail: tail,
        violations: issues.len(),
        digest: 0,
    })
}
