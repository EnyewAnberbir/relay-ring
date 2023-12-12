//! Integration test for `RR-0926` (basic).
//! Extended: Gate gateway ack replay extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0926_gate_gateway_ack_replay_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa7, 0xa9];
    let first = relayring::capabilities::rr_0926_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0926: Extended: Gate gateway ack replay extend codec v1");
    let second = relayring::capabilities::rr_0926_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0926: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0926: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0926: window consumes the whole buffer");
}
