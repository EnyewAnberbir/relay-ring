//! Integration test for `RR-0312` (roundtrip).
//! Gateway agent relay harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0312_gateway_agent_relay_hard_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3d, 0x3f];
    let a = relayring::capabilities::rr_0312_gateway_agent_relay_hard::evaluate(fixture).expect("RR-0312 first pass");
    let b = relayring::capabilities::rr_0312_gateway_agent_relay_hard::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
