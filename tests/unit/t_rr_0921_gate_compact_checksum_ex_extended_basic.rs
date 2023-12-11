//! Integration test for `RR-0921` (basic).
//! Extended: Gate compact checksum export export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0921_gate_compact_checksum_ex_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa2, 0xa4];
    let first = relayring::capabilities::rr_0921_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0921: Extended: Gate compact checksum export export adapter v26");
    let second = relayring::capabilities::rr_0921_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0921: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0921: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0921: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
