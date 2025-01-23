//! Integration test for `RR-0891` (roundtrip).
//! Extended: Gate surfaces seal index export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0891_gate_surfaces_seal_index_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x84, 0x86];
    let a = relayring::capabilities::rr_0891_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0891 first pass");
    let b = relayring::capabilities::rr_0891_gate_surfaces_seal_index_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
