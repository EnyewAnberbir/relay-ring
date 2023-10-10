//! Integration test for `RR-0447` (basic).
//! Gate gateway ack replay harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0447_gate_gateway_ack_replay_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc4, 0xc6];
    let first = relayring::capabilities::rr_0447_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0447: Gate gateway ack replay harden index v22");
    let second = relayring::capabilities::rr_0447_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0447: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0447: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0447: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
