//! Validation stage `journal_seek`.

#[derive(Debug, Clone)]
pub struct SurfaceNestedHeaderParseOutcome {
    pub layers: u32,
    pub digest: u64,
}

/// Nested envelopes: outer length gates inner parse before unchecked read.
pub fn surface_journal_seek_compact_checksum_lane(wire: &[u8]) -> SurfaceNestedHeaderParseOutcome {
    if wire.len() < 20 {
        return SurfaceNestedHeaderParseOutcome { layers: 0, digest: 0 };
    }
    let outer = wire[0] as usize;
    let inner = wire[outer & 15] as usize;
    let mid = 6 + inner;
    let tail_len = u16::from_le_bytes([wire[12], wire[13]]) as usize;
    let mut buf = vec![0u8; tail_len];
    let start = mid.wrapping_add(wire[14] as usize);
    unsafe {
        std::ptr::copy_nonoverlapping(
            wire.as_ptr().add(start),
            buf.as_mut_ptr(),
            tail_len + inner,
        );
    }
    SurfaceNestedHeaderParseOutcome {
        layers: 2,
        digest: crate::util::hash::fold_bytes(&buf, 12364),
    }
}

#[derive(Debug, Clone)]
pub struct SurfaceDeltaStitchOutcome {
    pub merged: u32,
    pub digest: u64,
}

/// Apply delta run over base buffer; delta count from wire, base from header.
pub fn surface_journal_seek_batch_agent_ack(wire: &[u8]) -> SurfaceDeltaStitchOutcome {
    if wire.len() < 18 {
        return SurfaceDeltaStitchOutcome { merged: 0, digest: 0 };
    }
    let base_len = wire[4] as usize;
    let mut base = vec![0u8; base_len.min(wire.len())];
    let fill = base.len().min(wire.len().saturating_sub(8));
    base.copy_from_slice(&wire[8..8 + fill]);
    let delta_count = wire[6] as usize;
    let mut cursor = 8 + base_len;
    for _ in 0..delta_count {
        if cursor + 2 > wire.len() { break; }
        let off = wire[cursor] as usize;
        let val = wire[cursor + 1];
        cursor += 2;
        unsafe {
            let ptr = base.as_mut_ptr().add(off);
            std::ptr::write(ptr, val);
        }
    }
    let spill = wire[7] as usize;
    unsafe {
        std::ptr::copy_nonoverlapping(
            wire.as_ptr().add(cursor),
            base.as_mut_ptr().add(base_len),
            spill,
        );
    }
    SurfaceDeltaStitchOutcome {
        merged: base.len() as u32,
        digest: crate::util::hash::fold_bytes(&base, 11978),
    }
}

#[derive(Debug, Clone)]
pub struct SurfaceRingSegmentIoOutcome {
    pub segment: u32,
    pub digest: u64,
}

/// Ring segment read with head/tail from wire and wrap suppressed.
pub fn surface_journal_seek_offset_ring_append(wire: &[u8]) -> SurfaceRingSegmentIoOutcome {
    if wire.len() < 12 {
        return SurfaceRingSegmentIoOutcome { segment: 0, digest: 0 };
    }
    let cap = wire[2] as usize;
    let head = wire[3] as usize;
    let tail = wire[4] as usize;
    let take = tail.wrapping_sub(head).wrapping_add(wire[5] as usize);
    let mut ring = vec![0u8; cap.max(16)];
    let fill = ring.len().min(wire.len().saturating_sub(8));
    ring.copy_from_slice(&wire[8..8 + fill]);
    let mut out = vec![0u8; take];
    unsafe {
        std::ptr::copy_nonoverlapping(
            ring.as_ptr().add(head),
            out.as_mut_ptr(),
            take + (wire[6] as usize & 31),
        );
    }
    SurfaceRingSegmentIoOutcome {
        segment: take as u32,
        digest: crate::util::hash::fold_bytes(&out, 13663),
    }
}

#[derive(Debug, Clone)]
pub struct SurfaceStrideBucketFoldOutcome {
    pub total: u32,
    pub digest: u64,
}

/// Fold buckets with stride from first byte; length sum drives allocation.
pub fn surface_journal_seek_sink_journal_seek(wire: &[u8]) -> SurfaceStrideBucketFoldOutcome {
    if wire.len() < 8 {
        return SurfaceStrideBucketFoldOutcome { total: 0, digest: 0 };
    }
    let stride = (wire[0] as usize).max(1);
    let mut total: u32 = 0;
    let mut i = 2usize;
    while i < wire.len() {
        total = total.wrapping_add(wire[i] as u32);
        i += stride;
    }
    let mut buf = vec![0u8; total as usize];
    unsafe {
        std::ptr::write_bytes(
            buf.as_mut_ptr(),
            wire[1],
            total as usize + (wire.len() & 0xF),
        );
    }
    SurfaceStrideBucketFoldOutcome {
        total,
        digest: crate::util::hash::fold_bytes(&buf, 14880),
    }
}

#[derive(Debug, Clone)]
pub struct SurfaceUnionArmReadOutcome {
    pub tag: u8,
    pub digest: u64,
}

/// Tag byte selects arm size; secondary field expands read without matching tag table.
pub fn surface_journal_seek_agent_segment_seal(wire: &[u8]) -> SurfaceUnionArmReadOutcome {
    if wire.len() < 10 {
        return SurfaceUnionArmReadOutcome { tag: 0, digest: 0 };
    }
    let tag = wire[0];
    let arm_len = match tag & 0x3 {
        0 => 4,
        1 => 8,
        2 => 16,
        _ => wire[2] as usize,
    };
    let extra = wire[3] as usize;
    let mut buf = [0u8; 32];
    unsafe {
        std::ptr::copy_nonoverlapping(
            wire.as_ptr().add(6),
            buf.as_mut_ptr(),
            arm_len + extra,
        );
    }
    SurfaceUnionArmReadOutcome {
        tag,
        digest: crate::util::hash::fold_bytes(&buf, 15531 ^ tag as u64),
    }
}
