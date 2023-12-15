//! Integration test for `RR-0942` (basic).
//! Extended: Gate gateway ack replay integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0942_gate_gateway_ack_replay_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb7, 0xb9];
    let first = relayring::capabilities::rr_0942_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0942: Extended: Gate gateway ack replay integrate validator v17");
    let second = relayring::capabilities::rr_0942_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0942: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0942: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0942: window consumes the whole buffer");
}
