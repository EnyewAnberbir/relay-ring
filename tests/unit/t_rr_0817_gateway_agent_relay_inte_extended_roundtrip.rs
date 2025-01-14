//! Integration test for `RR-0817` (roundtrip).
//! Extended: Gateway agent relay integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0817_gateway_agent_relay_inte_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3a, 0x3c];
    let a = relayring::capabilities::rr_0817_gateway_agent_relay_inte_extended::evaluate(fixture).expect("RR-0817 first pass");
    let b = relayring::capabilities::rr_0817_gateway_agent_relay_inte_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
