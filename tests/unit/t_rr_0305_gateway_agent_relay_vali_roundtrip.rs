//! Integration test for `RR-0305` (roundtrip).
//! Gateway agent relay validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0305_gateway_agent_relay_vali_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x36, 0x38];
    let a = relayring::capabilities::rr_0305_gateway_agent_relay_vali::evaluate(fixture).expect("RR-0305 first pass");
    let b = relayring::capabilities::rr_0305_gateway_agent_relay_vali::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
