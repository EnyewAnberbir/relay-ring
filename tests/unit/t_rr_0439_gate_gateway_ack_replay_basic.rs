//! Integration test for `RR-0439` (basic).
//! Gate gateway ack replay optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0439_gate_gateway_ack_replay_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbc, 0xbe];
    let first = relayring::capabilities::rr_0439_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0439: Gate gateway ack replay optimize registry v14");
    let second = relayring::capabilities::rr_0439_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0439: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0439: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0439: window consumes the whole buffer");
}
