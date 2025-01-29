//! Integration test for `RR-0937` (roundtrip).
//! Extended: Gate gateway ack replay harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0937_gate_gateway_ack_replay_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb2, 0xb4];
    let a = relayring::capabilities::rr_0937_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0937 first pass");
    let b = relayring::capabilities::rr_0937_gate_gateway_ack_replay_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
