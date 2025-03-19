//! Integration test for `RR-0301` (stability).
//! Gateway agent relay extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0301_gateway_agent_relay_exte_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x32, 0x34];
    let full = relayring::capabilities::rr_0301_gateway_agent_relay_exte::evaluate(fixture).expect("RR-0301: bulk Gateway agent relay extend codec v1");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0301_gateway_agent_relay_exte::evaluate(&fixture[..end]).expect("RR-0301: stable prefix");
        assert!(partial.consumed <= end, "RR-0301: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0301: full prefix should match bulk checksum");
        }
    }
}
