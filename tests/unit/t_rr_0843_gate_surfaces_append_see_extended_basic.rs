//! Integration test for `RR-0843` (basic).
//! Extended: Gate surfaces append seek refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0843_gate_surfaces_append_see_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x54, 0x56];
    let first = relayring::capabilities::rr_0843_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0843: Extended: Gate surfaces append seek refactor mutator v8");
    let second = relayring::capabilities::rr_0843_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0843: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0843: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0843: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
