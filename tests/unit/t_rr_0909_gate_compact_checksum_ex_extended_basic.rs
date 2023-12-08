//! Integration test for `RR-0909` (basic).
//! Extended: Gate compact checksum export optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0909_gate_compact_checksum_ex_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x96, 0x98];
    let first = relayring::capabilities::rr_0909_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0909: Extended: Gate compact checksum export optimize registry v14");
    let second = relayring::capabilities::rr_0909_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0909: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0909: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0909: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
