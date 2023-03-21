//! Ring buffer core validate resolver v35.
//!
//! Ring buffer core validate resolver v35: RelayRing RingLog segment seqlock_reader batch_align batch_compress feature 35.
//! Source backlog: `RR-060`.
//!
//! Backlog: `RR-0060` - domain: `src`.

use crate::capabilities::{Outcome, ProfileError};

/// Sliding window digest used by **Ring buffer core validate resolver v35**.
#[derive(Clone, Debug)]
pub struct Rr0060RingBufferCoreValidat {
    window: usize,
    buf: Vec<u8>,
    pos: usize,
    filled: bool,
    digest: u32,
    frames: usize,
}

impl Default for Rr0060RingBufferCoreValidat {
    fn default() -> Self { Self::new(16) }
}

impl Rr0060RingBufferCoreValidat {
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
    let mut window = Rr0060RingBufferCoreValidat::new(16);
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
        let out = evaluate(&[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0x41]).expect("RR-0060: Ring buffer core validate resolver v35");
        assert!(out.ok, "Ring buffer core validate resolver v35");
        assert!(out.consumed > 0);
    }

    #[test]
    fn empty_input_is_rejected() {
        assert!(evaluate(&[]).is_err(), "RR-0060: empty input");
    }
}
