//! Integration test for `RR-0440` (basic).
//! Gate gateway ack replay validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0440_gate_gateway_ack_replay_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbd, 0xbf];
    let first = relayring::capabilities::rr_0440_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0440: Gate gateway ack replay validate resolver v15");
    let second = relayring::capabilities::rr_0440_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0440: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0440: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0440: window consumes the whole buffer");
}
