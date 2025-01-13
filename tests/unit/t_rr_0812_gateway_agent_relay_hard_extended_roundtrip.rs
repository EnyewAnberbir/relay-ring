//! Integration test for `RR-0812` (roundtrip).
//! Extended: Gateway agent relay harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0812_gateway_agent_relay_hard_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x35, 0x37];
    let a = relayring::capabilities::rr_0812_gateway_agent_relay_hard_extended::evaluate(fixture).expect("RR-0812 first pass");
    let b = relayring::capabilities::rr_0812_gateway_agent_relay_hard_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
