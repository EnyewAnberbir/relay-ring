//! Integration test for `RR-0908` (basic).
//! Extended: Gate compact checksum export wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0908_gate_compact_checksum_ex_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x95, 0x97];
    let first = relayring::capabilities::rr_0908_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0908: Extended: Gate compact checksum export wire planner v13");
    let second = relayring::capabilities::rr_0908_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0908: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0908: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0908: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
