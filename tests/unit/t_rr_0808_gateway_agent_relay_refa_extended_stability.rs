//! Integration test for `RR-0808` (stability).
//! Extended: Gateway agent relay refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0808_gateway_agent_relay_refa_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x31, 0x33];
    let full = relayring::capabilities::rr_0808_gateway_agent_relay_refa_extended::evaluate(fixture).expect("RR-0808: bulk Extended: Gateway agent relay refactor mutator v8");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0808_gateway_agent_relay_refa_extended::evaluate(&fixture[..end]).expect("RR-0808: stable prefix");
        assert!(partial.consumed <= end, "RR-0808: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0808: full prefix should match bulk checksum");
        }
    }
}
