//! Integration test for `RR-0426` (basic).
//! Gate gateway ack replay extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0426_gate_gateway_ack_replay_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xaf, 0xb1];
    let first = relayring::capabilities::rr_0426_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0426: Gate gateway ack replay extend codec v1");
    let second = relayring::capabilities::rr_0426_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0426: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0426: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0426: window consumes the whole buffer");
}
