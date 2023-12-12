//! Integration test for `RR-0928` (basic).
//! Extended: Gate gateway ack replay wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0928_gate_gateway_ack_replay_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa9, 0xab];
    let first = relayring::capabilities::rr_0928_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0928: Extended: Gate gateway ack replay wire planner v3");
    let second = relayring::capabilities::rr_0928_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0928: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0928: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0928: window consumes the whole buffer");
}
