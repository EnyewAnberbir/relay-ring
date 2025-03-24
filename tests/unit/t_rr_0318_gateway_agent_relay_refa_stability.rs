//! Integration test for `RR-0318` (stability).
//! Gateway agent relay refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0318_gateway_agent_relay_refa_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x43, 0x45];
    let full = relayring::capabilities::rr_0318_gateway_agent_relay_refa::evaluate(fixture).expect("RR-0318: bulk Gateway agent relay refactor mutator v18");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0318_gateway_agent_relay_refa::evaluate(&fixture[..end]).expect("RR-0318: stable prefix");
        assert!(partial.consumed <= end, "RR-0318: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0318: full prefix should match bulk checksum");
        }
    }
}
