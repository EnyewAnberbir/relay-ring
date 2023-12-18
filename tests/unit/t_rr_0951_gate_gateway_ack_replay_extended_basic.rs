//! Integration test for `RR-0951` (basic).
//! Extended: Gate gateway ack replay export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0951_gate_gateway_ack_replay_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc0, 0xc2];
    let first = relayring::capabilities::rr_0951_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0951: Extended: Gate gateway ack replay export adapter v26");
    let second = relayring::capabilities::rr_0951_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0951: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0951: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0951: window consumes the whole buffer");
}
