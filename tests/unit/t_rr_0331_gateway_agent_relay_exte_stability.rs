//! Integration test for `RR-0331` (stability).
//! Gateway agent relay extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0331_gateway_agent_relay_exte_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x50, 0x52];
    let full = relayring::capabilities::rr_0331_gateway_agent_relay_exte::evaluate(fixture).expect("RR-0331: bulk Gateway agent relay extend codec v31");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0331_gateway_agent_relay_exte::evaluate(&fixture[..end]).expect("RR-0331: stable prefix");
        assert!(partial.consumed <= end, "RR-0331: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0331: full prefix should match bulk checksum");
        }
    }
}
