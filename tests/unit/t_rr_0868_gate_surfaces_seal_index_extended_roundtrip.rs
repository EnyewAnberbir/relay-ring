//! Integration test for `RR-0868` (roundtrip).
//! Extended: Gate surfaces seal index wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0868_gate_surfaces_seal_index_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6d, 0x6f];
    let a = relayring::capabilities::rr_0868_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0868 first pass");
    let b = relayring::capabilities::rr_0868_gate_surfaces_seal_index_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
