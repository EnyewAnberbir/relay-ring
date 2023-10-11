//! Integration test for `RR-0453` (basic).
//! Gate gateway ack replay refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0453_gate_gateway_ack_replay_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xca, 0xcc];
    let first = relayring::capabilities::rr_0453_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0453: Gate gateway ack replay refactor mutator v28");
    let second = relayring::capabilities::rr_0453_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0453: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0453: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0453: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
