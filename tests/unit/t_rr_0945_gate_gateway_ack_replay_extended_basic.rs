//! Integration test for `RR-0945` (basic).
//! Extended: Gate gateway ack replay implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0945_gate_gateway_ack_replay_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xba, 0xbc];
    let first = relayring::capabilities::rr_0945_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0945: Extended: Gate gateway ack replay implement pipeline v20");
    let second = relayring::capabilities::rr_0945_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0945: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0945: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0945: stats visits every byte");
}
