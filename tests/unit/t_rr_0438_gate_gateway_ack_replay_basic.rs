//! Integration test for `RR-0438` (basic).
//! Gate gateway ack replay wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0438_gate_gateway_ack_replay_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbb, 0xbd];
    let first = relayring::capabilities::rr_0438_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0438: Gate gateway ack replay wire planner v13");
    let second = relayring::capabilities::rr_0438_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0438: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0438: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0438: window consumes the whole buffer");
}
