//! Integration test for `RR-0943` (basic).
//! Extended: Gate gateway ack replay refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0943_gate_gateway_ack_replay_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb8, 0xba];
    let first = relayring::capabilities::rr_0943_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0943: Extended: Gate gateway ack replay refactor mutator v18");
    let second = relayring::capabilities::rr_0943_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0943: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0943: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0943: stats visits every byte");
}
