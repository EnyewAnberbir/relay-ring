//! Integration test for `RR-0828` (roundtrip).
//! Extended: Gateway agent relay refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0828_gateway_agent_relay_refa_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x45, 0x47];
    let a = relayring::capabilities::rr_0828_gateway_agent_relay_refa_extended::evaluate(fixture).expect("RR-0828 first pass");
    let b = relayring::capabilities::rr_0828_gateway_agent_relay_refa_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
