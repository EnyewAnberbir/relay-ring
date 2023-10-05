//! Integration test for `RR-0416` (basic).
//! Gate compact checksum export extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0416_gate_compact_checksum_ex_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa5, 0xa7];
    let first = relayring::capabilities::rr_0416_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0416: Gate compact checksum export extend codec v21");
    let second = relayring::capabilities::rr_0416_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0416: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0416: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0416: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
