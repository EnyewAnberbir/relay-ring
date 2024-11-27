//! Integration test for `RR-0451` (roundtrip).
//! Gate gateway ack replay export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0451_gate_gateway_ack_replay_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8, 0xca];
    let a = relayring::capabilities::rr_0451_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0451 first pass");
    let b = relayring::capabilities::rr_0451_gate_gateway_ack_replay::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
