//! Integration test for `RR-0808` (stream).
//! Extended: Gateway agent relay refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0808_gateway_agent_relay_refa_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x31, 0x33];
    let direct = relayring::capabilities::rr_0808_gateway_agent_relay_refa_extended::evaluate(fixture).expect("RR-0808: direct Extended: Gateway agent relay refactor mutator v8");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0808_gateway_agent_relay_refa_extended::evaluate(&copied).expect("RR-0808: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0808: stream path must consume input");
}
