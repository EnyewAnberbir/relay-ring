//! Integration test for `RR-0938` (basic).
//! Extended: Gate gateway ack replay wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0938_gate_gateway_ack_replay_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb3, 0xb5];
    let first = relayring::capabilities::rr_0938_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0938: Extended: Gate gateway ack replay wire planner v13");
    let second = relayring::capabilities::rr_0938_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0938: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0938: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0938: stats visits every byte");
}
