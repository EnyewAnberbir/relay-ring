//! Extended: Gate compact checksum export harden index v2.
//!
//! Extended variant derived from backlog RR-397: Gate compact checksum export harden index v2: surface_compact_pass surface_checksum_lane surface_export_batch feature 2.
//! Source backlog: `RR-397`.
//!
//! Backlog: `RR-0897` - domain: `src`.

use crate::capabilities::{Outcome, ProfileError};

/// Rolling statistics used by **Extended: Gate compact checksum export harden index v2**.
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

/// Accumulator implementing **Extended: Gate compact checksum export harden index v2**.
#[derive(Clone, Debug, Default)]
pub struct Rr0897GateCompactChecksumExExtended(Stats);

impl Rr0897GateCompactChecksumExExtended {
    pub fn new() -> Self { Self(Stats::default()) }
    pub fn ingest(&mut self, data: &[u8]) { for b in data { self.0.push(*b); } }
    pub fn stats(&self) -> &Stats { &self.0 }
}

/// Evaluate `data` and report accumulated statistics.
pub fn evaluate(data: &[u8]) -> Result<Outcome, ProfileError> {
    if data.is_empty() {
        return Err(ProfileError::EmptyInput);
    }
    let mut acc = Rr0897GateCompactChecksumExExtended::new();
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
        let out = evaluate(&[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8a, 0x8c]).expect("RR-0897: Extended: Gate compact checksum export harden index v2");
        assert!(out.ok, "Extended: Gate compact checksum export harden index v2");
        assert!(out.consumed > 0);
    }

    #[test]
    fn empty_input_is_rejected() {
        assert!(evaluate(&[]).is_err(), "RR-0897: empty input");
    }
}
