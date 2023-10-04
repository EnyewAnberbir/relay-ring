//! Integration test for `RR-0406` (basic).
//! Gate compact checksum export extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0406_gate_compact_checksum_ex_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9b, 0x9d];
    let first = relayring::capabilities::rr_0406_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0406: Gate compact checksum export extend codec v11");
    let second = relayring::capabilities::rr_0406_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0406: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0406: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0406: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
