//! Integration test for `RR-0913` (basic).
//! Extended: Gate compact checksum export refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0913_gate_compact_checksum_ex_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9a, 0x9c];
    let first = relayring::capabilities::rr_0913_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0913: Extended: Gate compact checksum export refactor mutator v18");
    let second = relayring::capabilities::rr_0913_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0913: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0913: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0913: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
