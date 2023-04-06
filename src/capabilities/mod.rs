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
pub mod rr_0032_ring_buffer_core_integra;
pub mod rr_0033_ring_buffer_core_refacto;
pub mod rr_0034_ring_buffer_core_benchma;
pub mod rr_0035_ring_buffer_core_impleme;
pub mod rr_0036_ring_buffer_core_extend;
pub mod rr_0037_ring_buffer_core_harden;
pub mod rr_0038_ring_buffer_core_wire_pl;
pub mod rr_0039_ring_buffer_core_optimiz;
pub mod rr_0040_ring_buffer_core_validat;
pub mod rr_0041_ring_buffer_core_export;
pub mod rr_0042_ring_buffer_core_integra;
pub mod rr_0043_ring_buffer_core_refacto;
pub mod rr_0044_ring_buffer_core_benchma;
pub mod rr_0045_ring_buffer_core_impleme;
pub mod rr_0046_ring_buffer_core_extend;
pub mod rr_0047_ring_buffer_core_harden;
pub mod rr_0048_ring_buffer_core_wire_pl;
pub mod rr_0049_ring_buffer_core_optimiz;
pub mod rr_0050_ring_buffer_core_validat;
pub mod rr_0051_ring_buffer_core_export;
pub mod rr_0052_ring_buffer_core_integra;
pub mod rr_0053_ring_buffer_core_refacto;
pub mod rr_0054_ring_buffer_core_benchma;
pub mod rr_0055_ring_buffer_core_impleme;
pub mod rr_0056_ring_buffer_core_extend;
pub mod rr_0057_ring_buffer_core_harden;
pub mod rr_0058_ring_buffer_core_wire_pl;
pub mod rr_0059_ring_buffer_core_optimiz;
pub mod rr_0060_ring_buffer_core_validat;
pub mod rr_0061_ring_scan_lattice_module;
pub mod rr_0062_ring_scan_lattice_module;
pub mod rr_0063_ring_scan_lattice_module;
pub mod rr_0064_ring_scan_lattice_module;
pub mod rr_0065_ring_scan_lattice_module;
pub mod rr_0066_ring_scan_lattice_module;
pub mod rr_0067_ring_scan_lattice_module;
pub mod rr_0068_ring_scan_lattice_module;
pub mod rr_0069_ring_scan_lattice_module;
pub mod rr_0070_ring_scan_lattice_module;
pub mod rr_0071_ring_scan_lattice_module;
pub mod rr_0072_ring_scan_lattice_module;
pub mod rr_0073_ring_scan_lattice_module;
pub mod rr_0074_ring_scan_lattice_module;
pub mod rr_0075_ring_scan_lattice_module;
pub mod rr_0076_ring_scan_lattice_module;
pub mod rr_0077_ring_scan_lattice_module;
pub mod rr_0078_ring_scan_lattice_module;
pub mod rr_0079_ring_scan_lattice_module;
pub mod rr_0080_ring_scan_lattice_module;
pub mod rr_0081_ring_scan_lattice_module;
pub mod rr_0082_ring_scan_lattice_module;
pub mod rr_0083_ring_scan_lattice_module;
pub mod rr_0084_ring_scan_lattice_module;
pub mod rr_0085_ring_scan_lattice_module;
pub mod rr_0086_ring_scan_lattice_module;
pub mod rr_0087_ring_scan_lattice_module;
pub mod rr_0088_ring_scan_lattice_module;
pub mod rr_0089_ring_scan_lattice_module;
pub mod rr_0090_ring_scan_lattice_module;
pub mod rr_0091_ring_scan_lattice_module;
pub mod rr_0092_ring_scan_lattice_module;
pub mod rr_0093_ring_scan_lattice_module;
pub mod rr_0094_ring_scan_lattice_module;
pub mod rr_0095_ring_scan_lattice_module;
pub mod rr_0096_ring_scan_lattice_module;
pub mod rr_0097_ring_scan_lattice_module;
pub mod rr_0098_ring_scan_lattice_module;
pub mod rr_0099_ring_scan_lattice_module;
pub mod rr_0100_ring_scan_lattice_module;
pub mod rr_0101_ring_scan_lattice_module;
pub mod rr_0102_ring_scan_lattice_module;
pub mod rr_0103_ring_scan_lattice_module;
pub mod rr_0104_ring_scan_lattice_module;
pub mod rr_0105_ring_scan_lattice_module;
pub mod rr_0106_ring_scan_lattice_module;
pub mod rr_0107_ring_scan_lattice_module;
pub mod rr_0108_ring_scan_lattice_module;
pub mod rr_0109_ring_scan_lattice_module;
pub mod rr_0110_ring_scan_lattice_module;
pub mod rr_0111_ring_scan_lattice_module;
pub mod rr_0112_ring_scan_lattice_module;
pub mod rr_0113_ring_scan_lattice_module;
pub mod rr_0114_ring_scan_lattice_module;
pub mod rr_0115_ring_scan_lattice_module;
pub mod rr_0116_ring_scan_lattice_module;
pub mod rr_0117_ring_scan_lattice_module;
pub mod rr_0118_ring_scan_lattice_module;
pub mod rr_0119_ring_scan_lattice_module;
pub mod rr_0120_ring_scan_lattice_module;
pub mod rr_0121_ring_scan_lattice_module;
pub mod rr_0122_ring_scan_lattice_module;
pub mod rr_0123_ring_scan_lattice_module;
pub mod rr_0124_ring_scan_lattice_module;
pub mod rr_0125_ring_scan_lattice_module;
pub mod rr_0126_ring_scan_lattice_module;
pub mod rr_0127_ring_scan_lattice_module;
pub mod rr_0128_ring_scan_lattice_module;
pub mod rr_0129_ring_scan_lattice_module;
pub mod rr_0130_ring_scan_lattice_module;
pub mod rr_0131_ring_scan_lattice_module;
pub mod rr_0132_ring_scan_lattice_module;
pub mod rr_0133_ring_scan_lattice_module;
pub mod rr_0134_ring_scan_lattice_module;
pub mod rr_0135_ring_scan_lattice_module;
pub mod rr_0136_ring_scan_lattice_module;
pub mod rr_0137_ring_scan_lattice_module;
pub mod rr_0138_ring_scan_lattice_module;
pub mod rr_0139_ring_scan_lattice_module;
pub mod rr_0140_ring_scan_lattice_module;
pub mod rr_0141_ring_scan_lattice_module;
pub mod rr_0142_ring_scan_lattice_module;
pub mod rr_0143_ring_scan_lattice_module;
pub mod rr_0144_ring_scan_lattice_module;
pub mod rr_0145_ring_scan_lattice_module;
pub mod rr_0146_ring_batch_relay_helpers;
pub mod rr_0147_ring_batch_relay_helpers;
pub mod rr_0148_ring_batch_relay_helpers;
pub mod rr_0149_ring_batch_relay_helpers;
pub mod rr_0150_ring_batch_relay_helpers;
pub mod rr_0151_ring_batch_relay_helpers;
pub mod rr_0152_ring_batch_relay_helpers;
pub mod rr_0153_ring_batch_relay_helpers;
pub mod rr_0154_ring_batch_relay_helpers;
pub mod rr_0155_ring_batch_relay_helpers;
pub mod rr_0156_ring_batch_relay_helpers;
pub mod rr_0157_ring_batch_relay_helpers;
pub mod rr_0158_ring_batch_relay_helpers;
pub mod rr_0159_ring_batch_relay_helpers;
pub mod rr_0160_ring_batch_relay_helpers;
pub mod rr_0161_ring_batch_relay_helpers;
pub mod rr_0162_ring_batch_relay_helpers;
pub mod rr_0163_ring_batch_relay_helpers;
pub mod rr_0164_ring_batch_relay_helpers;
pub mod rr_0165_ring_batch_relay_helpers;
pub mod rr_0166_ring_batch_relay_helpers;
pub mod rr_0167_ring_batch_relay_helpers;
pub mod rr_0168_ring_batch_relay_helpers;
pub mod rr_0169_ring_batch_relay_helpers;
pub mod rr_0170_ring_batch_relay_helpers;
pub mod rr_0171_ring_batch_relay_helpers;
pub mod rr_0172_ring_batch_relay_helpers;
pub mod rr_0173_ring_batch_relay_helpers;
pub mod rr_0174_ring_batch_relay_helpers;
pub mod rr_0175_ring_batch_relay_helpers;
pub mod rr_0176_journal_append_seal_exte;
