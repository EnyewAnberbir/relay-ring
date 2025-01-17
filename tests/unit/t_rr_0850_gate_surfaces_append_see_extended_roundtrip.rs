//! Integration test for `RR-0850` (roundtrip).
//! Extended: Gate surfaces append seek validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0850_gate_surfaces_append_see_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5b, 0x5d];
    let a = relayring::capabilities::rr_0850_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0850 first pass");
    let b = relayring::capabilities::rr_0850_gate_surfaces_append_see_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
