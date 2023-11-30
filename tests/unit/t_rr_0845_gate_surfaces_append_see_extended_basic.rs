//! Integration test for `RR-0845` (basic).
//! Extended: Gate surfaces append seek implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0845_gate_surfaces_append_see_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x56, 0x58];
    let first = relayring::capabilities::rr_0845_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0845: Extended: Gate surfaces append seek implement pipeline v10");
    let second = relayring::capabilities::rr_0845_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0845: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0845: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0845: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
