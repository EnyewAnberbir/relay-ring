//! Integration test for `RR-0431` (basic).
//! Gate gateway ack replay export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0431_gate_gateway_ack_replay_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb4, 0xb6];
    let first = relayring::capabilities::rr_0431_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0431: Gate gateway ack replay export adapter v6");
    let second = relayring::capabilities::rr_0431_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0431: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0431: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0431: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
