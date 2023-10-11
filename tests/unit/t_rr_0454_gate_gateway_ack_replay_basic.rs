//! Integration test for `RR-0454` (basic).
//! Gate gateway ack replay benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0454_gate_gateway_ack_replay_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcb, 0xcd];
    let first = relayring::capabilities::rr_0454_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0454: Gate gateway ack replay benchmark reporter v29");
    let second = relayring::capabilities::rr_0454_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0454: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0454: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0454: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
