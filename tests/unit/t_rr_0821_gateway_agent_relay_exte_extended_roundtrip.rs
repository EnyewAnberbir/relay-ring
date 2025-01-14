//! Integration test for `RR-0821` (roundtrip).
//! Extended: Gateway agent relay extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0821_gateway_agent_relay_exte_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3e, 0x40];
    let a = relayring::capabilities::rr_0821_gateway_agent_relay_exte_extended::evaluate(fixture).expect("RR-0821 first pass");
    let b = relayring::capabilities::rr_0821_gateway_agent_relay_exte_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
