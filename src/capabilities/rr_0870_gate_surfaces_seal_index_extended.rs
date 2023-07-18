//! Extended: Gate surfaces seal index validate resolver v5.
//!
//! Extended variant derived from backlog RR-370: Gate surfaces seal index validate resolver v5: surface_segment_seal surface_offset_index LeaseStackPop BitmapDenseFill feature 5.
//! Source backlog: `RR-370`.
//!
//! Backlog: `RR-0870` - domain: `src`.

use crate::capabilities::{Outcome, ProfileError};

/// Sliding window digest used by **Extended: Gate surfaces seal index validate resolver v5**.
#[derive(Clone, Debug)]
pub struct Rr0870GateSurfacesSealIndexExtended {
    window: usize,
    buf: Vec<u8>,
    pos: usize,
    filled: bool,
    digest: u32,
    frames: usize,
}

impl Default for Rr0870GateSurfacesSealIndexExtended {
    fn default() -> Self { Self::new(16) }
}

impl Rr0870GateSurfacesSealIndexExtended {
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
    let mut window = Rr0870GateSurfacesSealIndexExtended::new(16);
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
        let out = evaluate(&[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6f, 0x71]).expect("RR-0870: Extended: Gate surfaces seal index validate resolver v5");
        assert!(out.ok, "Extended: Gate surfaces seal index validate resolver v5");
        assert!(out.consumed > 0);
    }

    #[test]
    fn empty_input_is_rejected() {
        assert!(evaluate(&[]).is_err(), "RR-0870: empty input");
    }
}
