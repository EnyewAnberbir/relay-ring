//! Integration test for `RR-0948` (basic).
//! Extended: Gate gateway ack replay wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0948_gate_gateway_ack_replay_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbd, 0xbf];
    let first = relayring::capabilities::rr_0948_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0948: Extended: Gate gateway ack replay wire planner v23");
    let second = relayring::capabilities::rr_0948_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0948: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0948: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0948: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
