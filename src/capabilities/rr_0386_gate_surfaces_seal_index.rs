//! Gate surfaces seal index extend codec v21.
//!
//! Gate surfaces seal index extend codec v21: surface_segment_seal surface_offset_index LeaseStackPop BitmapDenseFill feature 21.
//! Source backlog: `RR-386`.
//!
//! Backlog: `RR-0386` - domain: `src`.

use crate::capabilities::{Outcome, ProfileError};

/// Rolling statistics used by **Gate surfaces seal index extend codec v21**.
#[derive(Clone, Debug, Default)]
pub struct Stats {
    pub samples: u64,
    pub sum: u64,
    pub min: u8,
    pub max: u8,
    pub digest: u32,
}

impl Stats {
    pub fn push(&mut self, byte: u8) {
        self.samples += 1;
        self.sum += byte as u64;
        if self.samples == 1 {
            self.min = byte;
            self.max = byte;
        } else {
            if byte < self.min { self.min = byte; }
            if byte > self.max { self.max = byte; }
        }
        self.digest ^= byte as u32;
        self.digest = self.digest.wrapping_mul(16_777_619);
    }

    pub fn mean(&self) -> f64 {
        if self.samples == 0 { 0.0 } else { self.sum as f64 / self.samples as f64 }
    }
}

/// Accumulator implementing **Gate surfaces seal index extend codec v21**.
#[derive(Clone, Debug, Default)]
pub struct Rr0386GateSurfacesSealIndex(Stats);

impl Rr0386GateSurfacesSealIndex {
    pub fn new() -> Self { Self(Stats::default()) }
    pub fn ingest(&mut self, data: &[u8]) { for b in data { self.0.push(*b); } }
    pub fn stats(&self) -> &Stats { &self.0 }
}

/// Evaluate `data` and report accumulated statistics.
pub fn evaluate(data: &[u8]) -> Result<Outcome, ProfileError> {
    if data.is_empty() {
        return Err(ProfileError::EmptyInput);
    }
    let mut acc = Rr0386GateSurfacesSealIndex::new();
    acc.ingest(data);
    Ok(Outcome {
        ok: acc.stats().samples > 0,
        consumed: data.len(),
        findings: acc.stats().samples as usize,
        checksum: acc.stats().digest,
        severity: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_fixture() {
        let out = evaluate(&[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x87, 0x89]).expect("RR-0386: Gate surfaces seal index extend codec v21");
        assert!(out.ok, "Gate surfaces seal index extend codec v21");
        assert!(out.consumed > 0);
    }

    #[test]
    fn empty_input_is_rejected() {
        assert!(evaluate(&[]).is_err(), "RR-0386: empty input");
    }
}
