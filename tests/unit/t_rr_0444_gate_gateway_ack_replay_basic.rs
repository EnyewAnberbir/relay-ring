//! Integration test for `RR-0444` (basic).
//! Gate gateway ack replay benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0444_gate_gateway_ack_replay_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc1, 0xc3];
    let first = relayring::capabilities::rr_0444_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0444: Gate gateway ack replay benchmark reporter v19");
    let second = relayring::capabilities::rr_0444_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0444: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0444: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0444: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
