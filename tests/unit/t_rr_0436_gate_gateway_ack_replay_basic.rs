//! Integration test for `RR-0436` (basic).
//! Gate gateway ack replay extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0436_gate_gateway_ack_replay_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb9, 0xbb];
    let first = relayring::capabilities::rr_0436_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0436: Gate gateway ack replay extend codec v11");
    let second = relayring::capabilities::rr_0436_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0436: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0436: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0436: stats visits every byte");
}
