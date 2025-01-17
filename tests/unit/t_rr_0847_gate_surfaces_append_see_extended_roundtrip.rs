//! Integration test for `RR-0847` (roundtrip).
//! Extended: Gate surfaces append seek harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0847_gate_surfaces_append_see_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x58, 0x5a];
    let a = relayring::capabilities::rr_0847_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0847 first pass");
    let b = relayring::capabilities::rr_0847_gate_surfaces_append_see_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
