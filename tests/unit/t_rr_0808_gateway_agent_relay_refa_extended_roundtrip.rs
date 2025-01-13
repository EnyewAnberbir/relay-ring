//! Integration test for `RR-0808` (roundtrip).
//! Extended: Gateway agent relay refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0808_gateway_agent_relay_refa_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x31, 0x33];
    let a = relayring::capabilities::rr_0808_gateway_agent_relay_refa_extended::evaluate(fixture).expect("RR-0808 first pass");
    let b = relayring::capabilities::rr_0808_gateway_agent_relay_refa_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
