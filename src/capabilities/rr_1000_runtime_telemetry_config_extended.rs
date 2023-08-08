//! Extended: Runtime telemetry config fuzz CLI validate resolver v45.
//!
//! Extended variant derived from backlog RR-500: Runtime telemetry config fuzz CLI validate resolver v45: engine_sequencer JournalReport ProfileRegistry HotRingSettings fuzz CLI feature 45.
//! Source backlog: `RR-500`.
//!
//! Backlog: `RR-1000` - domain: `src`.

use crate::capabilities::{Outcome, ProfileError};

/// Sliding window digest used by **Extended: Runtime telemetry config fuzz CLI validate resolver v45**.
#[derive(Clone, Debug)]
pub struct Rr1000RuntimeTelemetryConfigExtended {
    window: usize,
    buf: Vec<u8>,
    pos: usize,
    filled: bool,
    digest: u32,
    frames: usize,
}

impl Default for Rr1000RuntimeTelemetryConfigExtended {
    fn default() -> Self { Self::new(16) }
}

impl Rr1000RuntimeTelemetryConfigExtended {
    pub fn new(window: usize) -> Self {
        let window = window.max(4);
        Self { window, buf: vec![0; window], pos: 0, filled: false, digest: 2_166_136_261, frames: 0 }
    }

    pub fn push(&mut self, byte: u8) -> Option<u32> {
        self.buf[self.pos] = byte;
        self.pos = (self.pos + 1) % self.window;
        if self.pos == 0 { self.filled = true; }
        self.digest ^= byte as u32;
        self.digest = self.digest.rotate_left(5).wrapping_mul(2_654_435_761);
        if self.filled {
            self.frames += 1;
            Some(self.digest)
        } else {
            None
        }
    }

    pub fn ingest(&mut self, data: &[u8]) -> usize {
        let mut emitted = 0;
        for b in data {
            if self.push(*b).is_some() { emitted += 1; }
        }
        emitted
    }

    pub fn frames(&self) -> usize { self.frames }
    pub fn digest(&self) -> u32 { self.digest }
}

/// Evaluate `data` by feeding it through the sliding window.
pub fn evaluate(data: &[u8]) -> Result<Outcome, ProfileError> {
    if data.is_empty() {
        return Err(ProfileError::EmptyInput);
    }
    let mut window = Rr1000RuntimeTelemetryConfigExtended::new(16);
    let frames = window.ingest(data);
    Ok(Outcome {
        ok: true,
        consumed: data.len(),
        findings: frames,
        checksum: window.digest(),
        severity: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_fixture() {
        let out = evaluate(&[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf1, 0xf3]).expect("RR-1000: Extended: Runtime telemetry config fuzz CLI validate resolver v45");
        assert!(out.ok, "Extended: Runtime telemetry config fuzz CLI validate resolver v45");
        assert!(out.consumed > 0);
    }

    #[test]
    fn empty_input_is_rejected() {
        assert!(evaluate(&[]).is_err(), "RR-1000: empty input");
    }
}
