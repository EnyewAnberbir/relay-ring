//! Integration test for `RR-0808` (basic).
//! Extended: Gateway agent relay refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0808_gateway_agent_relay_refa_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x31, 0x33];
    let first = relayring::capabilities::rr_0808_gateway_agent_relay_refa_extended::evaluate(fixture).expect("RR-0808: Extended: Gateway agent relay refactor mutator v8");
    let second = relayring::capabilities::rr_0808_gateway_agent_relay_refa_extended::evaluate(fixture).expect("RR-0808: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0808: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0808: window consumes the whole buffer");
}
