//! Integration test for `RR-0365` (basic).
//! Gate surfaces append seek implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0365_gate_surfaces_append_see_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x72, 0x74];
    let first = relayring::capabilities::rr_0365_gate_surfaces_append_see::evaluate(fixture).expect("RR-0365: Gate surfaces append seek implement pipeline v30");
    let second = relayring::capabilities::rr_0365_gate_surfaces_append_see::evaluate(fixture).expect("RR-0365: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0365: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0365: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
