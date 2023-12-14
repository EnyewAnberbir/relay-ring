//! Integration test for `RR-0935` (basic).
//! Extended: Gate gateway ack replay implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0935_gate_gateway_ack_replay_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb0, 0xb2];
    let first = relayring::capabilities::rr_0935_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0935: Extended: Gate gateway ack replay implement pipeline v10");
    let second = relayring::capabilities::rr_0935_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0935: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0935: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0935: window consumes the whole buffer");
}
