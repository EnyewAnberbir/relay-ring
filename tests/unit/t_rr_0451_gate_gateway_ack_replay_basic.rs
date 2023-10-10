//! Integration test for `RR-0451` (basic).
//! Gate gateway ack replay export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0451_gate_gateway_ack_replay_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8, 0xca];
    let first = relayring::capabilities::rr_0451_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0451: Gate gateway ack replay export adapter v26");
    let second = relayring::capabilities::rr_0451_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0451: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0451: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0451: stats visits every byte");
}
