//! Integration test for `RR-0827` (stability).
//! Extended: Gateway agent relay integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0827_gateway_agent_relay_inte_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x44, 0x46];
    let full = relayring::capabilities::rr_0827_gateway_agent_relay_inte_extended::evaluate(fixture).expect("RR-0827: bulk Extended: Gateway agent relay integrate validator v27");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0827_gateway_agent_relay_inte_extended::evaluate(&fixture[..end]).expect("RR-0827: stable prefix");
        assert!(partial.consumed <= end, "RR-0827: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0827: full prefix should match bulk checksum");
        }
    }
}
