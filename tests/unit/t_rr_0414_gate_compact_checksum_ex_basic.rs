//! Integration test for `RR-0414` (basic).
//! Gate compact checksum export benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0414_gate_compact_checksum_ex_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa3, 0xa5];
    let first = relayring::capabilities::rr_0414_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0414: Gate compact checksum export benchmark reporter v19");
    let second = relayring::capabilities::rr_0414_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0414: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0414: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0414: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
