//! Integration test for `RR-0318` (roundtrip).
//! Gateway agent relay refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0318_gateway_agent_relay_refa_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x43, 0x45];
    let a = relayring::capabilities::rr_0318_gateway_agent_relay_refa::evaluate(fixture).expect("RR-0318 first pass");
    let b = relayring::capabilities::rr_0318_gateway_agent_relay_refa::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
