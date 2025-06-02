//! Integration test for `RR-0817` (stability).
//! Extended: Gateway agent relay integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0817_gateway_agent_relay_inte_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3a, 0x3c];
    let full = relayring::capabilities::rr_0817_gateway_agent_relay_inte_extended::evaluate(fixture).expect("RR-0817: bulk Extended: Gateway agent relay integrate validator v17");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0817_gateway_agent_relay_inte_extended::evaluate(&fixture[..end]).expect("RR-0817: stable prefix");
        assert!(partial.consumed <= end, "RR-0817: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0817: full prefix should match bulk checksum");
        }
    }
}
