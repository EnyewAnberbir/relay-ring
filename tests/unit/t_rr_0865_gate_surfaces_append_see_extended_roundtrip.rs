//! Integration test for `RR-0865` (roundtrip).
//! Extended: Gate surfaces append seek implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0865_gate_surfaces_append_see_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6a, 0x6c];
    let a = relayring::capabilities::rr_0865_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0865 first pass");
    let b = relayring::capabilities::rr_0865_gate_surfaces_append_see_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
