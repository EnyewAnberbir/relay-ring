//! Integration test for `RR-0314` (roundtrip).
//! Gateway agent relay optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0314_gateway_agent_relay_opti_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0x41];
    let a = relayring::capabilities::rr_0314_gateway_agent_relay_opti::evaluate(fixture).expect("RR-0314 first pass");
    let b = relayring::capabilities::rr_0314_gateway_agent_relay_opti::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
