//! Integration test for `RR-0950` (basic).
//! Extended: Gate gateway ack replay validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0950_gate_gateway_ack_replay_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbf, 0xc1];
    let first = relayring::capabilities::rr_0950_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0950: Extended: Gate gateway ack replay validate resolver v25");
    let second = relayring::capabilities::rr_0950_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0950: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0950: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0950: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
