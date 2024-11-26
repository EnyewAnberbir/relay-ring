//! Integration test for `RR-0439` (roundtrip).
//! Gate gateway ack replay optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0439_gate_gateway_ack_replay_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbc, 0xbe];
    let a = relayring::capabilities::rr_0439_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0439 first pass");
    let b = relayring::capabilities::rr_0439_gate_gateway_ack_replay::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
