//! Integration test for `RR-0346` (roundtrip).
//! Gate surfaces append seek extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0346_gate_surfaces_append_see_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5f, 0x61];
    let a = relayring::capabilities::rr_0346_gate_surfaces_append_see::evaluate(fixture).expect("RR-0346 first pass");
    let b = relayring::capabilities::rr_0346_gate_surfaces_append_see::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
