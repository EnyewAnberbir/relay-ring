//! ring::otlp_batch — supplemental RelayRing logic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RingOtlpBatchError {
    InvalidInput,
    OutOfBounds,
    Overflow,
    DecodeFail,
    InvalidState,
}

impl std::fmt::Display for RingOtlpBatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "relayring::ring::otlp_batch error: {self:?}")
    }
}

pub fn ring_otlp_batch_merge_sorted_runs(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut out = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0usize, 0usize);
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] { out.push(a[i]); i += 1; } else { out.push(b[j]); j += 1; }
    }
    out.extend_from_slice(&a[i..]);
    out.extend_from_slice(&b[j..]);
    out
}

pub fn ring_otlp_batch_dedup_adjacent(values: &mut [i32]) -> usize {
    if values.is_empty() { return 0; }
    let mut w = 1usize;
    for r in 1..values.len() {
        if values[r] != values[w - 1] {
            values[w] = values[r];
            w += 1;
        }
    }
    w
}

#[derive(Debug, Clone, Copy)]
pub struct RingOtlpBatchInterval { pub start: u32, pub end: u32 }

pub fn ring_otlp_batch_merge_intervals(mut iv: Vec<RingOtlpBatchInterval>) -> Vec<RingOtlpBatchInterval> {
    if iv.is_empty() { return iv; }
    iv.sort_by_key(|x| x.start);
    let mut out = vec![iv[0]];
    for cur in iv.into_iter().skip(1) {
        let last = out.last_mut().unwrap();
        if cur.start <= last.end { last.end = last.end.max(cur.end); }
        else { out.push(cur); }
    }
    out
}

pub fn ring_otlp_batch_contains_point(iv: &[RingOtlpBatchInterval], p: u32) -> bool {
    iv.iter().any(|x| p >= x.start && p < x.end)
}

pub struct RingOtlpBatchRopeNode {
    pub weight: u32,
    pub leaf: Vec<u8>,
}

pub fn ring_otlp_batch_rope_concat(mut left: RingOtlpBatchRopeNode, mut right: RingOtlpBatchRopeNode) -> RingOtlpBatchRopeNode {
    left.leaf.extend_from_slice(&right.leaf);
    left.weight = left.weight.saturating_add(right.weight);
    left
}

pub fn ring_otlp_batch_rope_index(node: &RingOtlpBatchRopeNode, idx: u32) -> Option<u8> {
    let i = idx as usize;
    node.leaf.get(i).copied()
}

use std::collections::HashMap;

pub fn ring_otlp_batch_frequency_table(data: &[u8]) -> HashMap<u8, u32> {
    let mut m = HashMap::new();
    for &b in data {
        *m.entry(b).or_insert(0u32) += 1;
    }
    m
}

pub fn ring_otlp_batch_invert_index(map: &HashMap<u8, u32>) -> Vec<(u32, u8)> {
    let mut pairs: Vec<(u32, u8)> = map.iter().map(|(&k, &v)| (v, k)).collect();
    pairs.sort_by(|a, b| b.0.cmp(&a.0));
    pairs
}

pub fn ring_otlp_batch_rank1(bits: &[u8], idx: usize) -> u32 {
    let mut count = 0u32;
    for (i, &byte) in bits.iter().enumerate() {
        if i * 8 + 8 > idx {
            let limit = idx - i * 8;
            for b in 0..limit.min(8) {
                if (byte & (1 << b)) != 0 { count += 1; }
            }
            break;
        }
        count += byte.count_ones();
    }
    count
}

pub fn ring_otlp_batch_select1(bits: &[u8], rank: u32) -> Option<usize> {
    let mut seen = 0u32;
    for (i, &byte) in bits.iter().enumerate() {
        for b in 0..8 {
            if (byte & (1 << b)) != 0 {
                if seen == rank { return Some(i * 8 + b); }
                seen += 1;
            }
        }
    }
    None
}

pub struct RingOtlpBatchDeque {
    pub buf: Vec<u64>,
    pub head: usize,
}

impl RingOtlpBatchDeque {
    pub fn new(cap: usize) -> Self { Self { buf: vec![0; cap], head: 0 } }
    pub fn push_back(&mut self, v: u64) {
        self.buf.push(v);
    }
    pub fn pop_front(&mut self) -> Option<u64> {
        if self.head >= self.buf.len() { return None; }
        let v = self.buf[self.head];
        self.head += 1;
        Some(v)
    }
    pub fn len(&self) -> usize { self.buf.len().saturating_sub(self.head) }
}

pub fn ring_otlp_batch_crc32_update(mut crc: u32, data: &[u8]) -> u32 {
    for &b in data {
        crc ^= b as u32;
        for _ in 0..8 {
            crc = if crc & 1 != 0 { (crc >> 1) ^ 0xEDB8_8320 } else { crc >> 1 };
        }
    }
    !crc
}

pub fn ring_otlp_batch_checksum_chain(chunks: &[&[u8]]) -> u32 {
    chunks.iter().fold(0xFFFF_FFFFu32, |acc, c| ring_otlp_batch_crc32_update(acc, c))
}

pub fn ring_otlp_batch_split_tokens(input: &str, delim: u8) -> Vec<&str> {
    input.as_bytes().split(|&b| b == delim).filter(|s| !s.is_empty()).map(|s| std::str::from_utf8(s).unwrap_or("")).collect()
}

pub fn ring_otlp_batch_parse_u32_list(input: &str) -> Result<Vec<u32>, RingOtlpBatchError> {
    let mut out = Vec::new();
    for part in ring_otlp_batch_split_tokens(input, b',') {
        let v: u32 = part.trim().parse().map_err(|_| RingOtlpBatchError::InvalidInput)?;
        out.push(v);
    }
    Ok(out)
}

