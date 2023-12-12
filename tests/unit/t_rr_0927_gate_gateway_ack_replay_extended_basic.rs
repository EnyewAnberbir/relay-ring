//! Integration test for `RR-0927` (basic).
//! Extended: Gate gateway ack replay harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0927_gate_gateway_ack_replay_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa8, 0xaa];
    let first = relayring::capabilities::rr_0927_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0927: Extended: Gate gateway ack replay harden index v2");
    let second = relayring::capabilities::rr_0927_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0927: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0927: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0927: scanner should emit domain hints");
}
