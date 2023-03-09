//! ring::scan_offset — supplemental RelayRing logic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RingScanOffsetError {
    InvalidInput,
    OutOfBounds,
    Overflow,
    DecodeFail,
    InvalidState,
}

impl std::fmt::Display for RingScanOffsetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "relayring::ring::scan_offset error: {self:?}")
    }
}

pub fn ring_scan_offset_merge_sorted_runs(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut out = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0usize, 0usize);
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] { out.push(a[i]); i += 1; } else { out.push(b[j]); j += 1; }
    }
    out.extend_from_slice(&a[i..]);
    out.extend_from_slice(&b[j..]);
    out
}

pub fn ring_scan_offset_dedup_adjacent(values: &mut [i32]) -> usize {
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
pub struct RingScanOffsetInterval { pub start: u32, pub end: u32 }

pub fn ring_scan_offset_merge_intervals(mut iv: Vec<RingScanOffsetInterval>) -> Vec<RingScanOffsetInterval> {
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

pub fn ring_scan_offset_contains_point(iv: &[RingScanOffsetInterval], p: u32) -> bool {
    iv.iter().any(|x| p >= x.start && p < x.end)
}

use std::collections::HashMap;

pub fn ring_scan_offset_frequency_table(data: &[u8]) -> HashMap<u8, u32> {
    let mut m = HashMap::new();
    for &b in data {
        *m.entry(b).or_insert(0u32) += 1;
    }
    m
}

pub fn ring_scan_offset_invert_index(map: &HashMap<u8, u32>) -> Vec<(u32, u8)> {
    let mut pairs: Vec<(u32, u8)> = map.iter().map(|(&k, &v)| (v, k)).collect();
    pairs.sort_by(|a, b| b.0.cmp(&a.0));
    pairs
}

pub struct RingScanOffsetDeque {
    pub buf: Vec<u64>,
    pub head: usize,
}

impl RingScanOffsetDeque {
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

pub fn ring_scan_offset_crc32_update(mut crc: u32, data: &[u8]) -> u32 {
    for &b in data {
        crc ^= b as u32;
        for _ in 0..8 {
            crc = if crc & 1 != 0 { (crc >> 1) ^ 0xEDB8_8320 } else { crc >> 1 };
        }
    }
    !crc
}

pub fn ring_scan_offset_checksum_chain(chunks: &[&[u8]]) -> u32 {
    chunks.iter().fold(0xFFFF_FFFFu32, |acc, c| ring_scan_offset_crc32_update(acc, c))
}

pub fn ring_scan_offset_encode_varint(mut v: u64, out: &mut Vec<u8>) {
    while v >= 0x80 {
        out.push((v as u8) | 0x80);
        v >>= 7;
    }
    out.push(v as u8);
}

pub fn ring_scan_offset_decode_varint(data: &[u8], off: &mut usize) -> Result<u64, RingScanOffsetError> {
    let mut shift = 0u32;
    let mut out = 0u64;
    loop {
        if *off >= data.len() { return Err(RingScanOffsetError::DecodeFail); }
        let b = data[*off];
        *off += 1;
        out |= ((b & 0x7F) as u64) << shift;
        if b & 0x80 == 0 { return Ok(out); }
        shift += 7;
        if shift > 63 { return Err(RingScanOffsetError::Overflow); }
    }
}

pub fn ring_scan_offset_split_tokens(input: &str, delim: u8) -> Vec<&str> {
    input.as_bytes().split(|&b| b == delim).filter(|s| !s.is_empty()).map(|s| std::str::from_utf8(s).unwrap_or("")).collect()
}

pub fn ring_scan_offset_parse_u32_list(input: &str) -> Result<Vec<u32>, RingScanOffsetError> {
    let mut out = Vec::new();
    for part in ring_scan_offset_split_tokens(input, b',') {
        let v: u32 = part.trim().parse().map_err(|_| RingScanOffsetError::InvalidInput)?;
        out.push(v);
    }
    Ok(out)
}

pub fn ring_scan_offset_topo_sort(n: usize, edges: &[(u32, u32)]) -> Option<Vec<u32>> {
    let mut indeg = vec![0u32; n];
    let mut adj: Vec<Vec<u32>> = vec![Vec::new(); n];
    for &(u, v) in edges {
        if (u as usize) < n && (v as usize) < n {
            adj[u as usize].push(v);
            indeg[v as usize] += 1;
        }
    }
    let mut q: Vec<u32> = (0..n as u32).filter(|&i| indeg[i as usize] == 0).collect();
    let mut order = Vec::with_capacity(n);
    while let Some(u) = q.pop() {
        order.push(u);
        for &v in &adj[u as usize] {
            indeg[v as usize] -= 1;
            if indeg[v as usize] == 0 { q.push(v); }
        }
    }
    if order.len() == n { Some(order) } else { None }
}

