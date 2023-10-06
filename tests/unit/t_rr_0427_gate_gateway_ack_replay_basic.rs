//! Integration test for `RR-0427` (basic).
//! Gate gateway ack replay harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0427_gate_gateway_ack_replay_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb0, 0xb2];
    let first = relayring::capabilities::rr_0427_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0427: Gate gateway ack replay harden index v2");
    let second = relayring::capabilities::rr_0427_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0427: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0427: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0427: window consumes the whole buffer");
}
