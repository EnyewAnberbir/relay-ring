//! Integration test for `RR-0448` (basic).
//! Gate gateway ack replay wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0448_gate_gateway_ack_replay_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc5, 0xc7];
    let first = relayring::capabilities::rr_0448_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0448: Gate gateway ack replay wire planner v23");
    let second = relayring::capabilities::rr_0448_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0448: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0448: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0448: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
