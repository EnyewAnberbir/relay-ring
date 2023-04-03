//! Ring scan lattice modules validate resolver v85.
//!
//! Ring scan lattice modules validate resolver v85: wire scan_span scan_chain scan_table scan_buffer scan_frame scan_batch feature 85.
//! Source backlog: `RR-145`.
//!
//! Backlog: `RR-0145` - domain: `src`.

use crate::capabilities::{Outcome, ProfileError};

/// Rolling statistics used by **Ring scan lattice modules validate resolver v85**.
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

/// Accumulator implementing **Ring scan lattice modules validate resolver v85**.
#[derive(Clone, Debug, Default)]
pub struct Rr0145RingScanLatticeModule(Stats);

impl Rr0145RingScanLatticeModule {
    pub fn new() -> Self { Self(Stats::default()) }
    pub fn ingest(&mut self, data: &[u8]) { for b in data { self.0.push(*b); } }
    pub fn stats(&self) -> &Stats { &self.0 }
}

/// Evaluate `data` and report accumulated statistics.
pub fn evaluate(data: &[u8]) -> Result<Outcome, ProfileError> {
    if data.is_empty() {
        return Err(ProfileError::EmptyInput);
    }
    let mut acc = Rr0145RingScanLatticeModule::new();
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
        let out = evaluate(&[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x94, 0x96]).expect("RR-0145: Ring scan lattice modules validate resolver v85");
        assert!(out.ok, "Ring scan lattice modules validate resolver v85");
        assert!(out.consumed > 0);
    }

    #[test]
    fn empty_input_is_rejected() {
        assert!(evaluate(&[]).is_err(), "RR-0145: empty input");
    }
}
