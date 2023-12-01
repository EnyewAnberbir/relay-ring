//! Integration test for `RR-0855` (basic).
//! Extended: Gate surfaces append seek implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0855_gate_surfaces_append_see_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x60, 0x62];
    let first = relayring::capabilities::rr_0855_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0855: Extended: Gate surfaces append seek implement pipeline v20");
    let second = relayring::capabilities::rr_0855_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0855: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0855: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0855: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
