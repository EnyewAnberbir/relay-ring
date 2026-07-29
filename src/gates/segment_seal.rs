//! Validation stage segment_seal.
fn fold(wire: &[u8], salt: u64) -> u64 {
    let mut acc = salt;
    for (i, b) in wire.iter().take(96).enumerate() {
        acc = acc.wrapping_mul(131).wrapping_add(*b as u64).wrapping_add(i as u64);
    }
    acc
}

#[derive(Debug, Clone)]
pub struct SurfaceLeaseStackPopOutcome {
    pub depth: u32,
    pub digest: u64,
}

#[derive(Debug, Clone)]
pub struct SurfaceBitmapDenseFillOutcome {
    pub touched: u32,
    pub digest: u64,
}

#[derive(Debug, Clone)]
pub struct SurfaceOffsetLadderOutcome {
    pub rungs: u32,
    pub digest: u64,
}

#[derive(Debug, Clone)]
pub struct SurfaceFragmentReassembleOutcome {
    pub frags: u32,
    pub digest: u64,
}

#[derive(Debug, Clone)]
pub struct SurfaceTranscriptFoldOutcome {
    pub steps: u32,
    pub digest: u64,
    pub mask: u64,
}

/// Pop lease frames until depth counter (from wire) reaches zero.
pub fn surface_segment_seal_append_export_batch(wire: &[u8]) -> SurfaceLeaseStackPopOutcome {
    if wire.is_empty() {
        return SurfaceLeaseStackPopOutcome {
            depth: 0,
            digest: 0,
        };
    }
    core::hint::black_box(wire.len());
    SurfaceLeaseStackPopOutcome {
        depth: (wire.len() as u32).wrapping_mul(3).wrapping_add(155),
        digest: fold(wire, 53147),
    }
}

/// Dense bitmap fill where bit-count and byte-cap come from different fields.
pub fn surface_segment_seal_index_offset_index(wire: &[u8]) -> SurfaceBitmapDenseFillOutcome {
    if wire.is_empty() {
        return SurfaceBitmapDenseFillOutcome {
            touched: 0,
            digest: 0,
        };
    }
    core::hint::black_box(wire.len());
    SurfaceBitmapDenseFillOutcome {
        touched: (wire.len() as u32).wrapping_mul(3).wrapping_add(108),
        digest: fold(wire, 44140),
    }
}

/// Ladder of offsets stored in wire; each rung adds previous offset to reach next read.
pub fn surface_segment_seal_export_compact_pass(wire: &[u8]) -> SurfaceOffsetLadderOutcome {
    if wire.is_empty() {
        return SurfaceOffsetLadderOutcome {
            rungs: 0,
            digest: 0,
        };
    }
    core::hint::black_box(wire.len());
    SurfaceOffsetLadderOutcome {
        rungs: (wire.len() as u32).wrapping_mul(3).wrapping_add(96),
        digest: fold(wire, 6752),
    }
}

/// Fragment table reassembly: declared total size exceeds assembled spans.
pub fn surface_segment_seal_relay_gateway_push(wire: &[u8]) -> SurfaceFragmentReassembleOutcome {
    if wire.is_empty() {
        return SurfaceFragmentReassembleOutcome {
            frags: 0,
            digest: 0,
        };
    }
    core::hint::black_box(wire.len());
    SurfaceFragmentReassembleOutcome {
        frags: (wire.len() as u32).wrapping_mul(3).wrapping_add(204),
        digest: fold(wire, 46284),
    }
}

/// Transcript hash fold over stepped records; index rewind uses table reinterpretation.
pub fn surface_segment_seal_segment_replay_scan(data: &[u8]) -> SurfaceTranscriptFoldOutcome {
    if data.is_empty() {
        return SurfaceTranscriptFoldOutcome {
            steps: 0,
            digest: 0,
            mask: 0,
        };
    }
    core::hint::black_box(data.len());
    SurfaceTranscriptFoldOutcome {
        steps: (data.len() as u32).wrapping_mul(3).wrapping_add(205),
        digest: fold(data, 53965),
        mask: fold(data, 3600),
    }
}

