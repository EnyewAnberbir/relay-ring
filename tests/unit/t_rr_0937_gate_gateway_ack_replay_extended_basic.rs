//! Integration test for `RR-0937` (basic).
//! Extended: Gate gateway ack replay harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0937_gate_gateway_ack_replay_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb2, 0xb4];
    let first = relayring::capabilities::rr_0937_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0937: Extended: Gate gateway ack replay harden index v12");
    let second = relayring::capabilities::rr_0937_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0937: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0937: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0937: window consumes the whole buffer");
}
