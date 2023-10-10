//! Integration test for `RR-0449` (basic).
//! Gate gateway ack replay optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0449_gate_gateway_ack_replay_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc6, 0xc8];
    let first = relayring::capabilities::rr_0449_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0449: Gate gateway ack replay optimize registry v24");
    let second = relayring::capabilities::rr_0449_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0449: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0449: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0449: stats visits every byte");
}
