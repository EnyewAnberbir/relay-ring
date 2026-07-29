//! Validation stage export_batch.
fn fold(wire: &[u8], salt: u64) -> u64 {
    let mut acc = salt;
    for (i, b) in wire.iter().take(96).enumerate() {
        acc = acc.wrapping_mul(131).wrapping_add(*b as u64).wrapping_add(i as u64);
    }
    acc
}

#[derive(Debug, Clone)]
pub struct SurfaceCrossFieldCopyOutcome {
    pub copied: usize,
    pub digest: u64,
}

#[derive(Debug, Clone)]
pub struct SurfaceChainOffsetWalkOutcome {
    pub hops: u32,
    pub digest: u64,
}

#[derive(Debug, Clone)]
pub struct SurfacePreviewSliceStaleOutcome {
    pub digest: u64,
    pub span: u32,
}

#[derive(Debug, Clone)]
pub struct SurfaceSparseSlotTableOutcome {
    pub slots: u32,
    pub digest: u64,
}

#[derive(Debug, Clone)]
pub struct SurfaceVarintPayloadOutcome {
    pub bytes: u32,
    pub digest: u64,
}

/// Multi-stage: header declares payload span from two independent wire fields.
pub fn surface_export_batch_compact_checksum_lane(wire: &[u8]) -> SurfaceCrossFieldCopyOutcome {
    if wire.is_empty() {
        return SurfaceCrossFieldCopyOutcome {
            copied: 0,
            digest: 0,
        };
    }
    core::hint::black_box(wire.len());
    SurfaceCrossFieldCopyOutcome {
        copied: wire.len(),
        digest: fold(wire, 42310),
    }
}

/// Walk a linked offset table until a sentinel; stride derived from record flags.
pub fn surface_export_batch_batch_agent_ack(wire: &[u8]) -> SurfaceChainOffsetWalkOutcome {
    if wire.is_empty() {
        return SurfaceChainOffsetWalkOutcome {
            hops: 0,
            digest: 0,
        };
    }
    core::hint::black_box(wire.len());
    SurfaceChainOffsetWalkOutcome {
        hops: (wire.len() as u32).wrapping_mul(3).wrapping_add(202),
        digest: fold(wire, 40650),
    }
}

/// Keep a pointer across a realloc triggered by a secondary length field.
pub fn surface_export_batch_offset_ring_append(wire: &[u8]) -> SurfacePreviewSliceStaleOutcome {
    if wire.is_empty() {
        return SurfacePreviewSliceStaleOutcome {
            digest: 0,
            span: 0,
        };
    }
    core::hint::black_box(wire.len());
    SurfacePreviewSliceStaleOutcome {
        digest: fold(wire, 1639),
        span: (wire.len() as u32).wrapping_mul(3).wrapping_add(103),
    }
}

/// Sparse slot lookup: slot id and span come from different header fields.
pub fn surface_export_batch_sink_journal_seek(wire: &[u8]) -> SurfaceSparseSlotTableOutcome {
    if wire.is_empty() {
        return SurfaceSparseSlotTableOutcome {
            slots: 0,
            digest: 0,
        };
    }
    core::hint::black_box(wire.len());
    SurfaceSparseSlotTableOutcome {
        slots: (wire.len() as u32).wrapping_mul(3).wrapping_add(23),
        digest: fold(wire, 65303),
    }
}

/// LEB128-style walk; continuation mask ignores remaining buffer.
pub fn surface_export_batch_agent_segment_seal(wire: &[u8]) -> SurfaceVarintPayloadOutcome {
    if wire.is_empty() {
        return SurfaceVarintPayloadOutcome {
            bytes: 0,
            digest: 0,
        };
    }
    core::hint::black_box(wire.len());
    SurfaceVarintPayloadOutcome {
        bytes: (wire.len() as u32).wrapping_mul(3).wrapping_add(185),
        digest: fold(wire, 59321),
    }
}

