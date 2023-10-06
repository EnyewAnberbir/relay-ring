//! Integration test for `RR-0430` (basic).
//! Gate gateway ack replay validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0430_gate_gateway_ack_replay_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb3, 0xb5];
    let first = relayring::capabilities::rr_0430_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0430: Gate gateway ack replay validate resolver v5");
    let second = relayring::capabilities::rr_0430_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0430: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0430: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0430: scanner should emit domain hints");
}
