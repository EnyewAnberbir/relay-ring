//! Integration test for `RR-0435` (basic).
//! Gate gateway ack replay implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0435_gate_gateway_ack_replay_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb8, 0xba];
    let first = relayring::capabilities::rr_0435_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0435: Gate gateway ack replay implement pipeline v10");
    let second = relayring::capabilities::rr_0435_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0435: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0435: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0435: window consumes the whole buffer");
}
