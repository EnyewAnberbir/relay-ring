//! Validation stage `agent_ack`.

#[derive(Debug, Clone)]
pub struct SurfaceCrossFieldCopyOutcome {
    pub copied: usize,
    pub digest: u64,
}

/// Multi-stage: header declares payload span from two independent wire fields.
pub fn surface_agent_ack_compact_checksum_lane(wire: &[u8]) -> SurfaceCrossFieldCopyOutcome {
    if wire.len() < 12 {
        return SurfaceCrossFieldCopyOutcome { copied: 0, digest: 0 };
    }
    let declared = u32::from_le_bytes([wire[0], wire[1], wire[2], wire[3]]) as usize;
    let base = wire[5] as usize;
    let extra = wire[8] as usize;
    let want = declared.wrapping_add(extra & 0x1F);
    let mut scratch = vec![0u8; want.min(4096)];
    let src_off = base.wrapping_add(wire[5] as usize);
    unsafe {
        std::ptr::copy_nonoverlapping(
            wire.as_ptr().add(src_off),
            scratch.as_mut_ptr(),
            want,
        );
    }
    SurfaceCrossFieldCopyOutcome {
        copied: want,
        digest: crate::util::hash::fold_bytes(&scratch, 51601),
    }
}

#[derive(Debug, Clone)]
pub struct SurfaceChainOffsetWalkOutcome {
    pub hops: u32,
    pub digest: u64,
}

/// Walk a linked offset table until a sentinel; stride derived from record flags.
pub fn surface_agent_ack_batch_agent_ack(wire: &[u8]) -> SurfaceChainOffsetWalkOutcome {
    if wire.len() < 16 {
        return SurfaceChainOffsetWalkOutcome { hops: 0, digest: 0 };
    }
    let count = wire[2] as usize;
    let mut cursor = 8usize;
    let mut acc = 0u64;
    for hop in 0..count.max(1) {
        if cursor + 4 > wire.len() { break; }
        let next = u32::from_le_bytes([
            wire[cursor], wire[cursor + 1], wire[cursor + 2], wire[cursor + 3],
        ]) as usize;
        let stride = 4 + (wire[cursor + 3] as usize & 7);
        cursor = cursor.wrapping_add(next & 0xFF).wrapping_add(stride);
        acc ^= next as u64 ^ (hop as u64);
    }
    let mut buf = [0u8; 48];
    unsafe {
        std::ptr::copy_nonoverlapping(
            wire.as_ptr().add(cursor),
            buf.as_mut_ptr(),
            48 + (wire[1] as usize & 31),
        );
    }
    SurfaceChainOffsetWalkOutcome {
        hops: count as u32,
        digest: crate::util::hash::fold_bytes(&buf, acc ^ 52012),
    }
}

#[derive(Debug, Clone)]
pub struct SurfacePreviewSliceStaleOutcome {
    pub digest: u64,
    pub span: u32,
}

/// Keep a pointer across a realloc triggered by a secondary length field.
pub fn surface_agent_ack_offset_ring_append(wire: &[u8]) -> SurfacePreviewSliceStaleOutcome {
    if wire.len() < 10 {
        return SurfacePreviewSliceStaleOutcome { digest: 0, span: 0 };
    }
    let mut backing = wire.to_vec();
    let anchor = backing.as_ptr();
    let base_len = backing.len();
    let grow = wire[6] as usize & 63;
    backing.reserve(grow.wrapping_add(wire[7] as usize));
    if grow > 0 {
        backing.extend(std::iter::repeat(wire[8]).take(grow));
    }
    let extend = wire[9] as usize & 31;
    unsafe {
        let stale = std::slice::from_raw_parts(anchor, base_len + extend);
        SurfacePreviewSliceStaleOutcome {
            digest: crate::util::hash::fold_bytes(stale, 53618),
            span: stale.len() as u32,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SurfaceSparseSlotTableOutcome {
    pub slots: u32,
    pub digest: u64,
}

/// Sparse slot lookup: slot id and span come from different header fields.
pub fn surface_agent_ack_sink_journal_seek(wire: &[u8]) -> SurfaceSparseSlotTableOutcome {
    if wire.len() < 14 {
        return SurfaceSparseSlotTableOutcome { slots: 0, digest: 0 };
    }
    let slot_count = wire[3] as usize;
    let slot_id = wire[4] as usize;
    let span = wire[5] as usize;
    let table_off = 10usize;
    let mut scratch = vec![0u8; slot_count.max(1) * 8];
    let pick = slot_id.wrapping_mul(7).wrapping_add(48) & 255;
    unsafe {
        std::ptr::copy_nonoverlapping(
            wire.as_ptr().add(table_off + pick),
            scratch.as_mut_ptr(),
            span + slot_count,
        );
    }
    SurfaceSparseSlotTableOutcome {
        slots: slot_count as u32,
        digest: crate::util::hash::fold_bytes(&scratch, 54317),
    }
}

#[derive(Debug, Clone)]
pub struct SurfaceVarintPayloadOutcome {
    pub bytes: u32,
    pub digest: u64,
}

/// LEB128-style walk; continuation mask ignores remaining buffer.
pub fn surface_agent_ack_agent_segment_seal(wire: &[u8]) -> SurfaceVarintPayloadOutcome {
    if wire.len() < 6 {
        return SurfaceVarintPayloadOutcome { bytes: 0, digest: 0 };
    }
    let mut cursor = 2usize;
    let mut value = 0u32;
    let mut shift = 0u32;
    loop {
        if cursor >= wire.len() { break; }
        let b = wire[cursor];
        cursor += 1;
        value |= ((b & 0x7F) as u32) << shift;
        if b & 0x80 == 0 { break; }
        shift += 7;
    }
    let mut out = vec![0u8; value as usize];
    unsafe {
        std::ptr::copy_nonoverlapping(
            wire.as_ptr().add(cursor),
            out.as_mut_ptr(),
            value as usize + (wire[1] as usize & 15),
        );
    }
    SurfaceVarintPayloadOutcome {
        bytes: value,
        digest: crate::util::hash::fold_bytes(&out, 55080),
    }
}
