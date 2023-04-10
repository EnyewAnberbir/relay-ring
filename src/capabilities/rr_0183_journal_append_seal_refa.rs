//! Journal append seal refactor mutator v8.
//!
//! Journal append seal refactor mutator v8: wire segment_index segment_seal sparse_index tail_hunter compact feature 8.
//! Source backlog: `RR-183`.
//!
//! Backlog: `RR-0183` - domain: `src`.

use crate::capabilities::{Outcome, ProfileError};

/// Validation finding with stable code and offset.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding {
    pub offset: usize,
    pub code: u16,
    pub message: &'static str,
}

/// Structural checker implementing **Journal append seal refactor mutator v8**.
#[derive(Clone, Debug)]
pub struct Rr0183JournalAppendSealRefa {
    min_len: usize,
    max_findings: usize,
}

impl Default for Rr0183JournalAppendSealRefa {
    fn default() -> Self { Self::new() }
}

impl Rr0183JournalAppendSealRefa {
    pub fn new() -> Self { Self { min_len: 4, max_findings: 32 } }

    pub fn require_min_length(mut self, n: usize) -> Self { self.min_len = n; self }

    /// Audit `data`, returning findings ordered by offset.
    pub fn audit(&self, data: &[u8]) -> Result<Vec<Finding>, ProfileError> {
        if data.is_empty() {
            return Err(ProfileError::EmptyInput);
        }
        let mut findings = Vec::new();
        if data.len() < self.min_len {
            findings.push(Finding { offset: 0, code: 100, message: "input shorter than minimum framing" });
        }
        for (i, b) in data.iter().enumerate() {
            if findings.len() >= self.max_findings { break; }
            if *b == 0 {
                findings.push(Finding { offset: i, code: 101, message: "embedded nul byte" });
            }
            if *b == b'\r' && i + 1 < data.len() && data[i + 1] == b'\n' && i > 0 && data[i - 1] == b'\r' {
                findings.push(Finding { offset: i, code: 102, message: "obs-fold sequence" });
            }
        }
        Ok(findings)
    }
}

/// Evaluate `data` and fold findings into an [`Outcome`].
pub fn evaluate(data: &[u8]) -> Result<Outcome, ProfileError> {
    let engine = Rr0183JournalAppendSealRefa::new();
    let findings = engine.audit(data)?;
    let severity = if findings.iter().any(|f| f.code >= 101) { 1 } else { 0 };
    let checksum = findings.iter().fold(0u32, |acc, f| acc.wrapping_add(f.code as u32));
    Ok(Outcome {
        ok: severity == 0,
        consumed: data.len(),
        findings: findings.len(),
        checksum,
        severity,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_fixture() {
        let out = evaluate(&[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xba, 0xbc]).expect("RR-0183: Journal append seal refactor mutator v8");
        assert!(out.ok, "Journal append seal refactor mutator v8");
        assert!(out.consumed > 0);
    }

    #[test]
    fn empty_input_is_rejected() {
        assert!(evaluate(&[]).is_err(), "RR-0183: empty input");
    }
}
