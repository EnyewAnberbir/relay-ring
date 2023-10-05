//! Integration test for `RR-0417` (basic).
//! Gate compact checksum export harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0417_gate_compact_checksum_ex_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa6, 0xa8];
    let first = relayring::capabilities::rr_0417_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0417: Gate compact checksum export harden index v22");
    let second = relayring::capabilities::rr_0417_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0417: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0417: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0417: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
