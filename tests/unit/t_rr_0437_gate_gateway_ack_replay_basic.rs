//! Integration test for `RR-0437` (basic).
//! Gate gateway ack replay harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0437_gate_gateway_ack_replay_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xba, 0xbc];
    let first = relayring::capabilities::rr_0437_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0437: Gate gateway ack replay harden index v12");
    let second = relayring::capabilities::rr_0437_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0437: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0437: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0437: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
