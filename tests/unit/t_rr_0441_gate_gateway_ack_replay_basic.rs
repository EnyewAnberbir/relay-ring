//! Integration test for `RR-0441` (basic).
//! Gate gateway ack replay export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0441_gate_gateway_ack_replay_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbe, 0xc0];
    let first = relayring::capabilities::rr_0441_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0441: Gate gateway ack replay export adapter v16");
    let second = relayring::capabilities::rr_0441_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0441: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0441: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0441: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
