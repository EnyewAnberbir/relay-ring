//! Integration test for `RR-0952` (basic).
//! Extended: Gate gateway ack replay integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0952_gate_gateway_ack_replay_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc1, 0xc3];
    let first = relayring::capabilities::rr_0952_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0952: Extended: Gate gateway ack replay integrate validator v27");
    let second = relayring::capabilities::rr_0952_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0952: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0952: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0952: stats visits every byte");
}
