//! Integration test for `RR-0349` (basic).
//! Gate surfaces append seek optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0349_gate_surfaces_append_see_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x62, 0x64];
    let first = relayring::capabilities::rr_0349_gate_surfaces_append_see::evaluate(fixture).expect("RR-0349: Gate surfaces append seek optimize registry v14");
    let second = relayring::capabilities::rr_0349_gate_surfaces_append_see::evaluate(fixture).expect("RR-0349: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0349: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0349: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
