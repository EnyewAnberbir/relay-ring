//! Integration test for `RR-0949` (basic).
//! Extended: Gate gateway ack replay optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0949_gate_gateway_ack_replay_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbe, 0xc0];
    let first = relayring::capabilities::rr_0949_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0949: Extended: Gate gateway ack replay optimize registry v24");
    let second = relayring::capabilities::rr_0949_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0949: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0949: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0949: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
