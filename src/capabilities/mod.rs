//! Capability modules extending `relayring` (see `docs/CAPABILITIES.md`).
//!
//! Each file implements one backlog item as a small, self-contained profile over
//! byte buffers. Callers use [`evaluate`] or the module-local scanner/engine type.

/// Unified evaluation outcome returned by capability modules.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Outcome {
    pub ok: bool,
    pub consumed: usize,
    pub findings: usize,
    pub checksum: u32,
    pub severity: u8,
}

/// Errors surfaced by capability modules.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProfileError {
    EmptyInput,
    Truncated { at: usize },
}

impl Outcome {
    pub fn is_clean(&self) -> bool { self.ok && self.severity == 0 }
}

pub mod rr_0001_wire_format_rlrg_frames;
pub mod rr_0002_wire_format_rlrg_frames;
pub mod rr_0003_wire_format_rlrg_frames;
pub mod rr_0004_wire_format_rlrg_frames;
pub mod rr_0005_wire_format_rlrg_frames;
pub mod rr_0006_wire_format_rlrg_frames;
pub mod rr_0007_wire_format_rlrg_frames;
pub mod rr_0008_wire_format_rlrg_frames;
pub mod rr_0009_wire_format_rlrg_frames;
pub mod rr_0010_wire_format_rlrg_frames;
pub mod rr_0011_wire_format_rlrg_frames;
pub mod rr_0012_wire_format_rlrg_frames;
pub mod rr_0013_wire_format_rlrg_frames;
pub mod rr_0014_wire_format_rlrg_frames;
pub mod rr_0015_wire_format_rlrg_frames;
pub mod rr_0016_wire_format_rlrg_frames;
pub mod rr_0017_wire_format_rlrg_frames;
pub mod rr_0018_wire_format_rlrg_frames;
pub mod rr_0019_wire_format_rlrg_frames;
pub mod rr_0020_wire_format_rlrg_frames;
pub mod rr_0021_wire_format_rlrg_frames;
pub mod rr_0022_wire_format_rlrg_frames;
pub mod rr_0023_wire_format_rlrg_frames;
pub mod rr_0024_wire_format_rlrg_frames;
pub mod rr_0025_wire_format_rlrg_frames;
pub mod rr_0026_ring_buffer_core_extend;
pub mod rr_0027_ring_buffer_core_harden;
pub mod rr_0028_ring_buffer_core_wire_pl;
pub mod rr_0029_ring_buffer_core_optimiz;
pub mod rr_0030_ring_buffer_core_validat;
pub mod rr_0031_ring_buffer_core_export;
