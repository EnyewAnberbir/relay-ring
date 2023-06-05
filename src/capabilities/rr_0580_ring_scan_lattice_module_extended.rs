//! Extended: Ring scan lattice modules implement pipeline v20.
//!
//! Extended variant derived from backlog RR-080: Ring scan lattice modules implement pipeline v20: wire scan_span scan_chain scan_table scan_buffer scan_frame scan_batch feature 20.
//! Source backlog: `RR-080`.
//!
//! Backlog: `RR-0580` - domain: `src`.

use crate::capabilities::{Outcome, ProfileError};

/// Parsed hint emitted while scanning a profile buffer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Hint {
    /// Span emitted when no domain-specific hint matched.
    Fallback(usize),
    Magic,
    SectionCount(usize),
    Version(u8),
}

/// Incremental profile scanner for **Extended: Ring scan lattice modules implement pipeline v20**.
#[derive(Clone, Debug)]
pub struct Rr0580RingScanLatticeModuleExtended {
    hints: Vec<Hint>,
    consumed: usize,
    rolling: u32,
}

/// Options controlling strictness and limits.
#[derive(Clone, Debug)]
pub struct Options {
    pub max_hints: usize,
    pub strict: bool,
}

impl Default for Options {
    fn default() -> Self { Self { max_hints: 64, strict: false } }
}

impl Default for Rr0580RingScanLatticeModuleExtended {
    fn default() -> Self { Self::new() }
}

impl Rr0580RingScanLatticeModuleExtended {
    /// Create an empty scanner with a fresh FNV rolling checksum.
    pub fn new() -> Self { Self { hints: Vec::new(), consumed: 0, rolling: 2_166_136_261 } }

    /// Scan `data`, appending hints and returning bytes consumed.
    pub fn scan(&mut self, data: &[u8], opts: Options) -> Result<usize, ProfileError> {
        if data.is_empty() {
            return Err(ProfileError::EmptyInput);
        }
        let strict = opts.strict;
        if strict && data.len() < 4 {
            return Err(ProfileError::Truncated { at: 0 });
        }
        let mut i = 0usize;
        while i < data.len() && self.hints.len() < opts.max_hints {
            self.rolling ^= data[i] as u32;
            self.rolling = self.rolling.wrapping_mul(16_777_619);
            i += 1;
        }
        if data.starts_with(b"RLRG") {
        self.hints.push(Hint::Magic);
        if data.len() >= 24 {
            let count = u32::from_le_bytes([data[24], data[25], data[26], data[27]]) as usize;
            self.hints.push(Hint::SectionCount(count));
        }
        if data.len() > 4 { self.hints.push(Hint::Version(data[4])); }
    }
        if self.hints.is_empty() {
            self.hints.push(Hint::Fallback(i));
        }
        self.consumed = i;
        Ok(i)
    }

    pub fn hints(&self) -> &[Hint] { &self.hints }
    pub fn checksum(&self) -> u32 { self.rolling }
    pub fn consumed(&self) -> usize { self.consumed }
}

/// Evaluate `data` and summarise the scan result.
pub fn evaluate(data: &[u8]) -> Result<Outcome, ProfileError> {
    let mut scanner = Rr0580RingScanLatticeModuleExtended::new();
    let consumed = scanner.scan(data, Options::default())?;
    Ok(Outcome {
        ok: true,
        consumed,
        findings: scanner.hints().len(),
        checksum: scanner.checksum(),
        severity: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discovers_hints_on_fixture() {
        let mut scanner = Rr0580RingScanLatticeModuleExtended::new();
        let fixture = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4b, 0x4d];
        let consumed = scanner.scan(fixture, Options::default()).expect("RR-0580: scan");
        assert!(consumed > 0);
        assert!(!scanner.hints().is_empty(), "Extended: Ring scan lattice modules implement pipeline v20");
        assert!(scanner.hints().iter().any(|h| matches!(h, Hint::Magic | Hint::SectionCount(_) | Hint::Version(_))), "expected rlrg hints");
    }

    #[test]
    fn empty_input_is_rejected() {
        assert!(evaluate(&[]).is_err(), "RR-0580: empty input");
    }
}
