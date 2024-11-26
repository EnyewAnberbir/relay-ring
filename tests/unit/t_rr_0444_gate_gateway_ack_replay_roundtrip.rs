//! Integration test for `RR-0444` (roundtrip).
//! Gate gateway ack replay benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0444_gate_gateway_ack_replay_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc1, 0xc3];
    let a = relayring::capabilities::rr_0444_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0444 first pass");
    let b = relayring::capabilities::rr_0444_gate_gateway_ack_replay::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
