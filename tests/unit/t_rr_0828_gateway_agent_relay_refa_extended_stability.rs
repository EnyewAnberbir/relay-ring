//! Integration test for `RR-0828` (stability).
//! Extended: Gateway agent relay refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0828_gateway_agent_relay_refa_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x45, 0x47];
    let full = relayring::capabilities::rr_0828_gateway_agent_relay_refa_extended::evaluate(fixture).expect("RR-0828: bulk Extended: Gateway agent relay refactor mutator v28");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0828_gateway_agent_relay_refa_extended::evaluate(&fixture[..end]).expect("RR-0828: stable prefix");
        assert!(partial.consumed <= end, "RR-0828: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0828: full prefix should match bulk checksum");
        }
    }
}
