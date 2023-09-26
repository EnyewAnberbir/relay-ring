//! Integration test for `RR-0341` (basic).
//! Gate surfaces append seek export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0341_gate_surfaces_append_see_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5a, 0x5c];
    let first = relayring::capabilities::rr_0341_gate_surfaces_append_see::evaluate(fixture).expect("RR-0341: Gate surfaces append seek export adapter v6");
    let second = relayring::capabilities::rr_0341_gate_surfaces_append_see::evaluate(fixture).expect("RR-0341: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0341: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0341: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
