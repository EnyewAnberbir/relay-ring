//! Integration test for `RR-0443` (basic).
//! Gate gateway ack replay refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0443_gate_gateway_ack_replay_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc0, 0xc2];
    let first = relayring::capabilities::rr_0443_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0443: Gate gateway ack replay refactor mutator v18");
    let second = relayring::capabilities::rr_0443_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0443: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0443: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0443: scanner should emit domain hints");
}
