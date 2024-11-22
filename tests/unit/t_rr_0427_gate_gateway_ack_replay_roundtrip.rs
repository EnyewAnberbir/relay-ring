//! Integration test for `RR-0427` (roundtrip).
//! Gate gateway ack replay harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0427_gate_gateway_ack_replay_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb0, 0xb2];
    let a = relayring::capabilities::rr_0427_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0427 first pass");
    let b = relayring::capabilities::rr_0427_gate_gateway_ack_replay::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
