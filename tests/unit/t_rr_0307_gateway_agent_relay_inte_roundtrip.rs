//! Integration test for `RR-0307` (roundtrip).
//! Gateway agent relay integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0307_gateway_agent_relay_inte_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x38, 0x3a];
    let a = relayring::capabilities::rr_0307_gateway_agent_relay_inte::evaluate(fixture).expect("RR-0307 first pass");
    let b = relayring::capabilities::rr_0307_gateway_agent_relay_inte::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
