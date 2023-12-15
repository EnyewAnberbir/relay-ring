//! Integration test for `RR-0944` (basic).
//! Extended: Gate gateway ack replay benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0944_gate_gateway_ack_replay_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb9, 0xbb];
    let first = relayring::capabilities::rr_0944_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0944: Extended: Gate gateway ack replay benchmark reporter v19");
    let second = relayring::capabilities::rr_0944_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0944: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0944: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0944: window consumes the whole buffer");
}
