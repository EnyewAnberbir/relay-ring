//! export::otlp_batch_builder — supplemental RelayRing logic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportOtlpBatchBuilderError {
    InvalidInput,
    OutOfBounds,
    Overflow,
    DecodeFail,
    InvalidState,
}

impl std::fmt::Display for ExportOtlpBatchBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "relayring::export::otlp_batch_builder error: {self:?}")
    }
}

pub fn export_otlp_batch_builder_merge_sorted_runs(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut out = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0usize, 0usize);
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] { out.push(a[i]); i += 1; } else { out.push(b[j]); j += 1; }
    }
    out.extend_from_slice(&a[i..]);
    out.extend_from_slice(&b[j..]);
    out
}

pub fn export_otlp_batch_builder_dedup_adjacent(values: &mut [i32]) -> usize {
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

pub fn export_otlp_batch_builder_top_k_by_abs(input: &[i32], k: usize) -> Vec<i32> {
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

pub fn export_otlp_batch_builder_select_nth_smallest(data: &mut [i32], n: usize) -> Result<i32, ExportOtlpBatchBuilderError> {
    if n >= data.len() { return Err(ExportOtlpBatchBuilderError::OutOfBounds); }
    data.select_nth_unstable(n);
    Ok(data[n])
}

#[derive(Debug, Clone, Copy)]
pub struct ExportOtlpBatchBuilderInterval { pub start: u32, pub end: u32 }

pub fn export_otlp_batch_builder_merge_intervals(mut iv: Vec<ExportOtlpBatchBuilderInterval>) -> Vec<ExportOtlpBatchBuilderInterval> {
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

pub fn export_otlp_batch_builder_contains_point(iv: &[ExportOtlpBatchBuilderInterval], p: u32) -> bool {
    iv.iter().any(|x| p >= x.start && p < x.end)
}

use std::collections::HashMap;

pub fn export_otlp_batch_builder_frequency_table(data: &[u8]) -> HashMap<u8, u32> {
    let mut m = HashMap::new();
    for &b in data {
        *m.entry(b).or_insert(0u32) += 1;
    }
    m
}

pub fn export_otlp_batch_builder_invert_index(map: &HashMap<u8, u32>) -> Vec<(u32, u8)> {
    let mut pairs: Vec<(u32, u8)> = map.iter().map(|(&k, &v)| (v, k)).collect();
    pairs.sort_by(|a, b| b.0.cmp(&a.0));
    pairs
}

pub struct ExportOtlpBatchBuilderDeque {
    pub buf: Vec<u64>,
    pub head: usize,
}

impl ExportOtlpBatchBuilderDeque {
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

pub fn export_otlp_batch_builder_encode_varint(mut v: u64, out: &mut Vec<u8>) {
    while v >= 0x80 {
        out.push((v as u8) | 0x80);
        v >>= 7;
    }
    out.push(v as u8);
}

pub fn export_otlp_batch_builder_decode_varint(data: &[u8], off: &mut usize) -> Result<u64, ExportOtlpBatchBuilderError> {
    let mut shift = 0u32;
    let mut out = 0u64;
    loop {
        if *off >= data.len() { return Err(ExportOtlpBatchBuilderError::DecodeFail); }
        let b = data[*off];
        *off += 1;
        out |= ((b & 0x7F) as u64) << shift;
        if b & 0x80 == 0 { return Ok(out); }
        shift += 7;
        if shift > 63 { return Err(ExportOtlpBatchBuilderError::Overflow); }
    }
}

pub fn export_otlp_batch_builder_split_tokens(input: &str, delim: u8) -> Vec<&str> {
    input.as_bytes().split(|&b| b == delim).filter(|s| !s.is_empty()).map(|s| std::str::from_utf8(s).unwrap_or("")).collect()
}

pub fn export_otlp_batch_builder_parse_u32_list(input: &str) -> Result<Vec<u32>, ExportOtlpBatchBuilderError> {
    let mut out = Vec::new();
    for part in export_otlp_batch_builder_split_tokens(input, b',') {
        let v: u32 = part.trim().parse().map_err(|_| ExportOtlpBatchBuilderError::InvalidInput)?;
        out.push(v);
    }
    Ok(out)
}

pub fn export_otlp_batch_builder_topo_sort(n: usize, edges: &[(u32, u32)]) -> Option<Vec<u32>> {
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

