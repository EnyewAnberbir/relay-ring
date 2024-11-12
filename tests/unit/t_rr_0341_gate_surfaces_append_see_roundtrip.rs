//! Integration test for `RR-0341` (roundtrip).
//! Gate surfaces append seek export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0341_gate_surfaces_append_see_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5a, 0x5c];
    let a = relayring::capabilities::rr_0341_gate_surfaces_append_see::evaluate(fixture).expect("RR-0341 first pass");
    let b = relayring::capabilities::rr_0341_gate_surfaces_append_see::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
