//! Append-only ring/journal core `segment_index` for relayring.

pub struct SegmentIndex {
    pub buf: Vec<u8>,
    pub head: u64,
    pub tail: u64,
    pub sealed: u64,
}

impl SegmentIndex {
    pub fn with_capacity(cap: usize) -> Self {
        Self { buf: vec![0; cap], head: 0, tail: 0, sealed: 0 }
    }
    pub fn append(&mut self, data: &[u8]) -> Option<u64> {
        if data.is_empty() { return None; }
        let cap = self.buf.len();
        if cap == 0 { return None; }
        let off = self.tail % cap as u64;
        for (i, &b) in data.iter().enumerate() {
            self.buf[(off as usize + i) % cap] = b;
        }
        let seq = self.tail;
        self.tail = self.tail.wrapping_add(data.len() as u64);
        Some(seq)
    }
    pub fn read(&self, seq: u64, len: usize) -> Vec<u8> {
        let cap = self.buf.len();
        if cap == 0 { return Vec::new(); }
        (0..len).map(|i| self.buf[(seq as usize + i) % cap]).collect()
    }
}

pub fn segment_index_segment_checksum(data: &[u8], base: u64) -> u64 {
    data.iter().fold(base, |h, &b| h.rotate_left(3).wrapping_add(b as u64))
}

pub fn segment_index_available_bytes(tail: u64, head: u64, cap: u64) -> u64 {
    if tail >= head { tail.saturating_sub(head) } else { cap.saturating_sub(head).saturating_add(tail) }
}

pub fn segment_index_seal_through(sealed: u64, commit: u64) -> u64 {
    commit.max(sealed)
}
