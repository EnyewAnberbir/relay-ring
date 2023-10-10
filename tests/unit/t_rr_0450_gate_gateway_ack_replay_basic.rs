//! Integration test for `RR-0450` (basic).
//! Gate gateway ack replay validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0450_gate_gateway_ack_replay_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc7, 0xc9];
    let first = relayring::capabilities::rr_0450_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0450: Gate gateway ack replay validate resolver v25");
    let second = relayring::capabilities::rr_0450_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0450: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0450: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0450: stats visits every byte");
}
