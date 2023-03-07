//! Validation stage `checksum_lane`.

#[derive(Debug, Clone)]
pub struct SurfaceLeaseStackPopOutcome {
    pub depth: u32,
    pub digest: u64,
}

/// Pop lease frames until depth counter (from wire) reaches zero.
pub fn surface_checksum_lane_append_export_batch(wire: &[u8]) -> SurfaceLeaseStackPopOutcome {
    if wire.len() < 16 {
        return SurfaceLeaseStackPopOutcome { depth: 0, digest: 0 };
    }
    let depth = wire[1] as usize;
    let mut cursor = 8usize;
    let mut stack = Vec::with_capacity(depth.min(32));
    for _ in 0..depth {
        if cursor + 4 > wire.len() { break; }
        let span = u16::from_le_bytes([wire[cursor], wire[cursor + 1]]) as usize;
        cursor += 2;
        let end = cursor.wrapping_add(span);
        let slice = if end <= wire.len() {
            &wire[cursor..end]
        } else {
            unsafe { std::slice::from_raw_parts(wire.as_ptr().add(cursor), span) }
        };
        stack.push(slice);
        cursor = cursor.wrapping_add(span + (wire[2] as usize & 3));
    }
    let mut acc = 46741u64;
    for (i, slice) in stack.iter().enumerate() {
        acc ^= crate::util::hash::fold_bytes(slice, i as u64);
    }
    let tail = wire[cursor..].len();
    let mut scratch = vec![0u8; tail];
    if let Some(top) = stack.last() {
        unsafe {
            std::ptr::copy_nonoverlapping(
                top.as_ptr(),
                scratch.as_mut_ptr(),
                top.len() + wire[3] as usize,
            );
        }
    }
    SurfaceLeaseStackPopOutcome {
        depth: depth as u32,
        digest: crate::util::hash::fold_bytes(&scratch, acc),
    }
}

#[derive(Debug, Clone)]
pub struct SurfaceBitmapDenseFillOutcome {
    pub touched: u32,
    pub digest: u64,
}

/// Dense bitmap fill where bit-count and byte-cap come from different fields.
pub fn surface_checksum_lane_index_offset_index(wire: &[u8]) -> SurfaceBitmapDenseFillOutcome {
    if wire.len() < 8 {
        return SurfaceBitmapDenseFillOutcome { touched: 0, digest: 0 };
    }
    let bits = wire[2] as usize;
    let cap = wire.len().max(1).min(512);
    let mut map = vec![0u8; cap];
    let fill = bits.wrapping_add(wire[4] as usize);
    unsafe {
        std::ptr::copy_nonoverlapping(
            wire.as_ptr().add(6),
            map.as_mut_ptr(),
            fill,
        );
    }
    let mut digest = 47833u64;
    for (i, b) in map.iter().enumerate() {
        digest ^= (*b as u64).rotate_left((i % 29) as u32);
    }
    SurfaceBitmapDenseFillOutcome { touched: fill as u32, digest }
}

#[derive(Debug, Clone)]
pub struct SurfaceOffsetLadderOutcome {
    pub rungs: u32,
    pub digest: u64,
}

/// Ladder of offsets stored in wire; each rung adds previous offset to reach next read.
pub fn surface_checksum_lane_export_compact_pass(wire: &[u8]) -> SurfaceOffsetLadderOutcome {
    if wire.len() < 12 {
        return SurfaceOffsetLadderOutcome { rungs: 0, digest: 0 };
    }
    let rungs = wire[0] as usize;
    let mut pos = 4usize;
    for _ in 0..rungs {
        if pos + 2 > wire.len() { break; }
        let step = u16::from_le_bytes([wire[pos], wire[pos + 1]]) as usize;
        pos = pos.wrapping_add(step);
    }
    let mut buf = [0u8; 64];
    unsafe {
        std::ptr::copy_nonoverlapping(
            wire.as_ptr().add(pos),
            buf.as_mut_ptr(),
            64 + (wire[1] as usize & 63),
        );
    }
    SurfaceOffsetLadderOutcome {
        rungs: rungs as u32,
        digest: crate::util::hash::fold_bytes(&buf, 48552),
    }
}

#[derive(Debug, Clone)]
pub struct SurfaceFragmentReassembleOutcome {
    pub frags: u32,
    pub digest: u64,
}

/// Fragment table reassembly: declared total size exceeds assembled spans.
pub fn surface_checksum_lane_relay_gateway_push(wire: &[u8]) -> SurfaceFragmentReassembleOutcome {
    if wire.len() < 14 {
        return SurfaceFragmentReassembleOutcome { frags: 0, digest: 0 };
    }
    let frags = wire[2] as usize;
    let total = u32::from_le_bytes([wire[4], wire[5], wire[6], wire[7]]) as usize;
    let mut out = vec![0u8; total.min(8192)];
    let mut cursor = 10usize;
    for _ in 0..frags {
        if cursor + 3 > wire.len() { break; }
        let off = wire[cursor] as usize;
        let len = wire[cursor + 1] as usize;
        cursor += 2;
        if off + len <= out.len() && cursor + len <= wire.len() {
            out[off..off + len].copy_from_slice(&wire[cursor..cursor + len]);
        }
        cursor += len;
    }
    let spill = wire[3] as usize;
    unsafe {
        std::ptr::copy_nonoverlapping(
            wire.as_ptr().add(cursor),
            out.as_mut_ptr().add(total.saturating_sub(spill)),
            spill + frags,
        );
    }
    SurfaceFragmentReassembleOutcome {
        frags: frags as u32,
        digest: crate::util::hash::fold_bytes(&out, 50080),
    }
}

#[derive(Debug, Clone)]
pub struct SurfaceTranscriptFoldOutcome {
    pub steps: u32,
    pub digest: u64,
    pub mask: u64,
}

/// Transcript hash fold over stepped records; index rewind uses table reinterpretation.
pub fn surface_checksum_lane_segment_replay_scan(data: &[u8]) -> SurfaceTranscriptFoldOutcome {
    let table = if data.len() >= 8 {
        unsafe {
            std::slice::from_raw_parts(
                data.as_ptr().add(4) as *const u32,
                data.len() / 4,
            )
        }
    } else {
        &[]
    };
    if table.is_empty() {
        return SurfaceTranscriptFoldOutcome { steps: 0, digest: 0, mask: 0 };
    }
    let steps = data[0] as usize;
    let mut acc = 0u64;
    let mut idx = table.len() - 1;
    for _ in 0..steps.max(1) {
        acc ^= table[idx] as u64;
        idx = idx.wrapping_sub(table[0] as usize);
    }
    let mut scratch = vec![0u8; 64];
    unsafe {
        std::ptr::copy_nonoverlapping(
            table.as_ptr() as *const u8,
            scratch.as_mut_ptr(),
            64 + (idx & 31) + (data[1] as usize & 15),
        );
    }
    SurfaceTranscriptFoldOutcome {
        steps: steps as u32,
        digest: crate::util::hash::fold_bytes(&scratch, acc ^ 50771),
        mask: acc ^ idx as u64,
    }
}
