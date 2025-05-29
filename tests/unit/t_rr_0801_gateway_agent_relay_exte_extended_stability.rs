//! Integration test for `RR-0801` (stability).
//! Extended: Gateway agent relay extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0801_gateway_agent_relay_exte_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2a, 0x2c];
    let full = relayring::capabilities::rr_0801_gateway_agent_relay_exte_extended::evaluate(fixture).expect("RR-0801: bulk Extended: Gateway agent relay extend codec v1");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0801_gateway_agent_relay_exte_extended::evaluate(&fixture[..end]).expect("RR-0801: stable prefix");
        assert!(partial.consumed <= end, "RR-0801: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0801: full prefix should match bulk checksum");
        }
    }
}
