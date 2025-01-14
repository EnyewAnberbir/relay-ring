//! Integration test for `RR-0822` (roundtrip).
//! Extended: Gateway agent relay harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0822_gateway_agent_relay_hard_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0x41];
    let a = relayring::capabilities::rr_0822_gateway_agent_relay_hard_extended::evaluate(fixture).expect("RR-0822 first pass");
    let b = relayring::capabilities::rr_0822_gateway_agent_relay_hard_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
