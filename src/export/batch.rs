//! export::batch — RelayRing domain logic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportBatchError {
    InvalidInput,
    OutOfBounds,
    Overflow,
    DecodeFail,
    InvalidState,
}

impl std::fmt::Display for ExportBatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "relayring error: {self:?}")
    }
}
pub struct RelayRing {
    pub buf: Vec<u8>,
    pub head: usize,
    pub tail: usize,
}

impl RelayRing {
    pub fn new(cap: usize) -> Self { Self { buf: vec![0; cap.max(8)], head: 0, tail: 0 } }
}

pub fn export_batch_ring_push(r: &mut RelayRing, b: u8) -> bool {
    let next = (r.tail + 1) % r.buf.len();
    if next == r.head { return false; }
    r.buf[r.tail] = b;
    r.tail = next;
    true
}

pub fn export_batch_ring_pop(r: &mut RelayRing) -> Option<u8> {
    if r.head == r.tail { return None; }
    let b = r.buf[r.head];
    r.head = (r.head + 1) % r.buf.len();
    Some(b)
}

pub fn export_batch_journal_offset_commit(base: u64, delta: u32) -> u64 {
    base.saturating_add(delta as u64)
}

pub fn export_batch_export_batch_pack(items: &[u16]) -> Vec<u8> {
    let mut out = Vec::with_capacity(items.len() * 2);
    for &x in items {
        out.extend_from_slice(&x.to_le_bytes());
    }
    out
}

pub fn export_batch_export_batch_unpack(data: &[u8]) -> Vec<u16> {
    data.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect()
}

pub fn export_batch_gateway_seq_next(seq: u64) -> u64 {
    seq.wrapping_add(1)
}

pub fn export_batch_gateway_seq_gap(prev: u64, next: u64) -> u64 {
    next.saturating_sub(prev)
}

pub fn export_batch_ring_len(r: &RelayRing) -> usize {
    if r.tail >= r.head { r.tail - r.head } else { r.buf.len() - r.head + r.tail }
}

pub fn export_batch_ring_drain(r: &mut RelayRing, n: usize) -> Vec<u8> {
    let mut out = Vec::new();
    for _ in 0..n {
        if let Some(b) = export_batch_ring_pop(r) { out.push(b); } else { break; }
    }
    out
}


pub fn export_batch_fold_le32(data: &[u8]) -> u32 {
    data.iter().take(4).enumerate().fold(0u32, |acc, (i, &b)| acc | ((b as u32) << (i * 8)))
}

pub fn export_batch_count_zeros(data: &[u8]) -> u32 {
    data.iter().filter(|&&b| b == 0).count() as u32
}

pub fn export_batch_take_prefix(data: &[u8], n: usize) -> &[u8] {
    &data[..n.min(data.len())]
}

pub fn export_batch_pair_hash(a: u32, b: u32) -> u64 {
    ((a as u64) << 32) | b as u64
}

pub fn export_batch_align_up(v: u32, a: u32) -> u32 {
    let al = a.max(1);
    (v + al - 1) / al * al
}

pub fn export_batch_contains_needle(hay: &[u8], needle: &[u8]) -> bool {
    hay.windows(needle.len()).any(|w| w == needle)
}