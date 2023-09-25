//! Integration test for `RR-0336` (basic).
//! Gate surfaces append seek extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0336_gate_surfaces_append_see_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x55, 0x57];
    let first = relayring::capabilities::rr_0336_gate_surfaces_append_see::evaluate(fixture).expect("RR-0336: Gate surfaces append seek extend codec v1");
    let second = relayring::capabilities::rr_0336_gate_surfaces_append_see::evaluate(fixture).expect("RR-0336: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0336: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0336: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
