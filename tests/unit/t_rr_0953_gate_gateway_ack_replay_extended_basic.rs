//! Integration test for `RR-0953` (basic).
//! Extended: Gate gateway ack replay refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0953_gate_gateway_ack_replay_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc2, 0xc4];
    let first = relayring::capabilities::rr_0953_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0953: Extended: Gate gateway ack replay refactor mutator v28");
    let second = relayring::capabilities::rr_0953_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0953: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0953: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0953: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
