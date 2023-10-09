//! Integration test for `RR-0434` (basic).
//! Gate gateway ack replay benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0434_gate_gateway_ack_replay_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb7, 0xb9];
    let first = relayring::capabilities::rr_0434_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0434: Gate gateway ack replay benchmark reporter v9");
    let second = relayring::capabilities::rr_0434_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0434: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0434: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0434: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
