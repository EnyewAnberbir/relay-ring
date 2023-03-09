//! ring::scan_run — supplemental RelayRing logic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RingScanRunError {
    InvalidInput,
    OutOfBounds,
    Overflow,
    DecodeFail,
    InvalidState,
}

impl std::fmt::Display for RingScanRunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "relayring::ring::scan_run error: {self:?}")
    }
}

pub fn ring_scan_run_top_k_by_abs(input: &[i32], k: usize) -> Vec<i32> {
    let mut heap: Vec<i32> = input.iter().copied().take(k).collect();
    heap.sort_by_key(|v| v.wrapping_abs());
    for &v in input.iter().skip(k) {
        if heap.is_empty() { heap.push(v); continue; }
        let worst = heap[0].wrapping_abs();
        if v.wrapping_abs() > worst {
            heap[0] = v;
            heap.sort_by_key(|x| x.wrapping_abs());
        }
    }
    heap
}

pub fn ring_scan_run_select_nth_smallest(data: &mut [i32], n: usize) -> Result<i32, RingScanRunError> {
    if n >= data.len() { return Err(RingScanRunError::OutOfBounds); }
    data.select_nth_unstable(n);
    Ok(data[n])
}

pub struct RingScanRunRopeNode {
    pub weight: u32,
    pub leaf: Vec<u8>,
}

pub fn ring_scan_run_rope_concat(mut left: RingScanRunRopeNode, mut right: RingScanRunRopeNode) -> RingScanRunRopeNode {
    left.leaf.extend_from_slice(&right.leaf);
    left.weight = left.weight.saturating_add(right.weight);
    left
}

pub fn ring_scan_run_rope_index(node: &RingScanRunRopeNode, idx: u32) -> Option<u8> {
    let i = idx as usize;
    node.leaf.get(i).copied()
}

use std::collections::HashMap;

pub fn ring_scan_run_frequency_table(data: &[u8]) -> HashMap<u8, u32> {
    let mut m = HashMap::new();
    for &b in data {
        *m.entry(b).or_insert(0u32) += 1;
    }
    m
}

pub fn ring_scan_run_invert_index(map: &HashMap<u8, u32>) -> Vec<(u32, u8)> {
    let mut pairs: Vec<(u32, u8)> = map.iter().map(|(&k, &v)| (v, k)).collect();
    pairs.sort_by(|a, b| b.0.cmp(&a.0));
    pairs
}

pub fn ring_scan_run_rank1(bits: &[u8], idx: usize) -> u32 {
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

pub fn ring_scan_run_select1(bits: &[u8], rank: u32) -> Option<usize> {
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

pub struct RingScanRunDeque {
    pub buf: Vec<u64>,
    pub head: usize,
}

impl RingScanRunDeque {
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

pub fn ring_scan_run_crc32_update(mut crc: u32, data: &[u8]) -> u32 {
    for &b in data {
        crc ^= b as u32;
        for _ in 0..8 {
            crc = if crc & 1 != 0 { (crc >> 1) ^ 0xEDB8_8320 } else { crc >> 1 };
        }
    }
    !crc
}

pub fn ring_scan_run_checksum_chain(chunks: &[&[u8]]) -> u32 {
    chunks.iter().fold(0xFFFF_FFFFu32, |acc, c| ring_scan_run_crc32_update(acc, c))
}

pub fn ring_scan_run_encode_varint(mut v: u64, out: &mut Vec<u8>) {
    while v >= 0x80 {
        out.push((v as u8) | 0x80);
        v >>= 7;
    }
    out.push(v as u8);
}

pub fn ring_scan_run_decode_varint(data: &[u8], off: &mut usize) -> Result<u64, RingScanRunError> {
    let mut shift = 0u32;
    let mut out = 0u64;
    loop {
        if *off >= data.len() { return Err(RingScanRunError::DecodeFail); }
        let b = data[*off];
        *off += 1;
        out |= ((b & 0x7F) as u64) << shift;
        if b & 0x80 == 0 { return Ok(out); }
        shift += 7;
        if shift > 63 { return Err(RingScanRunError::Overflow); }
    }
}

pub fn ring_scan_run_topo_sort(n: usize, edges: &[(u32, u32)]) -> Option<Vec<u32>> {
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

