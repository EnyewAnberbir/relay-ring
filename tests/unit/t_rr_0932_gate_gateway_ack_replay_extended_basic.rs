//! Integration test for `RR-0932` (basic).
//! Extended: Gate gateway ack replay integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0932_gate_gateway_ack_replay_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xad, 0xaf];
    let first = relayring::capabilities::rr_0932_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0932: Extended: Gate gateway ack replay integrate validator v7");
    let second = relayring::capabilities::rr_0932_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0932: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0932: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0932: scanner should emit domain hints");
}
