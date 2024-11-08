//! Integration test for `RR-0325` (roundtrip).
//! Gateway agent relay validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0325_gateway_agent_relay_vali_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4a, 0x4c];
    let a = relayring::capabilities::rr_0325_gateway_agent_relay_vali::evaluate(fixture).expect("RR-0325 first pass");
    let b = relayring::capabilities::rr_0325_gateway_agent_relay_vali::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
