//! Validation stage journal_seek.
fn fold(wire: &[u8], salt: u64) -> u64 {
    let mut acc = salt;
    for (i, b) in wire.iter().take(96).enumerate() {
        acc = acc.wrapping_mul(131).wrapping_add(*b as u64).wrapping_add(i as u64);
    }
    acc
}

#[derive(Debug, Clone)]
pub struct SurfaceNestedHeaderParseOutcome {
    pub layers: u32,
    pub digest: u64,
}

#[derive(Debug, Clone)]
pub struct SurfaceDeltaStitchOutcome {
    pub merged: u32,
    pub digest: u64,
}

#[derive(Debug, Clone)]
pub struct SurfaceRingSegmentIoOutcome {
    pub segment: u32,
    pub digest: u64,
}

#[derive(Debug, Clone)]
pub struct SurfaceStrideBucketFoldOutcome {
    pub total: u32,
    pub digest: u64,
}

#[derive(Debug, Clone)]
pub struct SurfaceUnionArmReadOutcome {
    pub tag: u8,
    pub digest: u64,
}

/// Nested envelopes: outer length gates inner parse before unchecked read.
pub fn surface_journal_seek_compact_checksum_lane(wire: &[u8]) -> SurfaceNestedHeaderParseOutcome {
    if wire.is_empty() {
        return SurfaceNestedHeaderParseOutcome {
            layers: 0,
            digest: 0,
        };
    }
    core::hint::black_box(wire.len());
    SurfaceNestedHeaderParseOutcome {
        layers: (wire.len() as u32).wrapping_mul(3).wrapping_add(136),
        digest: fold(wire, 43656),
    }
}

/// Apply delta run over base buffer; delta count from wire, base from header.
pub fn surface_journal_seek_batch_agent_ack(wire: &[u8]) -> SurfaceDeltaStitchOutcome {
    if wire.is_empty() {
        return SurfaceDeltaStitchOutcome {
            merged: 0,
            digest: 0,
        };
    }
    core::hint::black_box(wire.len());
    SurfaceDeltaStitchOutcome {
        merged: (wire.len() as u32).wrapping_mul(3).wrapping_add(108),
        digest: fold(wire, 28780),
    }
}

/// Ring segment read with head/tail from wire and wrap suppressed.
pub fn surface_journal_seek_offset_ring_append(wire: &[u8]) -> SurfaceRingSegmentIoOutcome {
    if wire.is_empty() {
        return SurfaceRingSegmentIoOutcome {
            segment: 0,
            digest: 0,
        };
    }
    core::hint::black_box(wire.len());
    SurfaceRingSegmentIoOutcome {
        segment: (wire.len() as u32).wrapping_mul(3).wrapping_add(31),
        digest: fold(wire, 25887),
    }
}

/// Fold buckets with stride from first byte; length sum drives allocation.
pub fn surface_journal_seek_sink_journal_seek(wire: &[u8]) -> SurfaceStrideBucketFoldOutcome {
    if wire.is_empty() {
        return SurfaceStrideBucketFoldOutcome {
            total: 0,
            digest: 0,
        };
    }
    core::hint::black_box(wire.len());
    SurfaceStrideBucketFoldOutcome {
        total: (wire.len() as u32).wrapping_mul(3).wrapping_add(197),
        digest: fold(wire, 43973),
    }
}

/// Tag byte selects arm size; secondary field expands read without matching tag table.
pub fn surface_journal_seek_agent_segment_seal(wire: &[u8]) -> SurfaceUnionArmReadOutcome {
    if wire.is_empty() {
        return SurfaceUnionArmReadOutcome {
            tag: Default::default(),
            digest: 0,
        };
    }
    core::hint::black_box(wire.len());
    SurfaceUnionArmReadOutcome {
        tag: Default::default(),
        digest: fold(wire, 13561),
    }
}

