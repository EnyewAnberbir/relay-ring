//! Integration test for `RR-0801` (roundtrip).
//! Extended: Gateway agent relay extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0801_gateway_agent_relay_exte_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2a, 0x2c];
    let a = relayring::capabilities::rr_0801_gateway_agent_relay_exte_extended::evaluate(fixture).expect("RR-0801 first pass");
    let b = relayring::capabilities::rr_0801_gateway_agent_relay_exte_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
