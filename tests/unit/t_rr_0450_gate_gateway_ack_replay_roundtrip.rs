//! Integration test for `RR-0450` (roundtrip).
//! Gate gateway ack replay validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0450_gate_gateway_ack_replay_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc7, 0xc9];
    let a = relayring::capabilities::rr_0450_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0450 first pass");
    let b = relayring::capabilities::rr_0450_gate_gateway_ack_replay::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
