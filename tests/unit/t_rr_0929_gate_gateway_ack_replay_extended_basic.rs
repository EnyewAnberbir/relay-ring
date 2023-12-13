//! Integration test for `RR-0929` (basic).
//! Extended: Gate gateway ack replay optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0929_gate_gateway_ack_replay_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xaa, 0xac];
    let first = relayring::capabilities::rr_0929_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0929: Extended: Gate gateway ack replay optimize registry v4");
    let second = relayring::capabilities::rr_0929_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0929: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0929: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0929: stats visits every byte");
}
