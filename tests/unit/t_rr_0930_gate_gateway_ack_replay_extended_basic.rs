//! Integration test for `RR-0930` (basic).
//! Extended: Gate gateway ack replay validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0930_gate_gateway_ack_replay_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xab, 0xad];
    let first = relayring::capabilities::rr_0930_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0930: Extended: Gate gateway ack replay validate resolver v5");
    let second = relayring::capabilities::rr_0930_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0930: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0930: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0930: stats visits every byte");
}
