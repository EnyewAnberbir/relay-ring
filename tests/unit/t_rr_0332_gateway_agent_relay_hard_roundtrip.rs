//! Integration test for `RR-0332` (roundtrip).
//! Gateway agent relay harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0332_gateway_agent_relay_hard_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x51, 0x53];
    let a = relayring::capabilities::rr_0332_gateway_agent_relay_hard::evaluate(fixture).expect("RR-0332 first pass");
    let b = relayring::capabilities::rr_0332_gateway_agent_relay_hard::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
