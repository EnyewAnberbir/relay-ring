//! Integration test for `RR-0947` (basic).
//! Extended: Gate gateway ack replay harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0947_gate_gateway_ack_replay_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbc, 0xbe];
    let first = relayring::capabilities::rr_0947_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0947: Extended: Gate gateway ack replay harden index v22");
    let second = relayring::capabilities::rr_0947_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0947: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0947: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0947: scanner should emit domain hints");
}
