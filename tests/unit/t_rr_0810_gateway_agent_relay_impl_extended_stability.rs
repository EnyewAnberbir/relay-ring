//! Integration test for `RR-0810` (stability).
//! Extended: Gateway agent relay implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0810_gateway_agent_relay_impl_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x33, 0x35];
    let full = relayring::capabilities::rr_0810_gateway_agent_relay_impl_extended::evaluate(fixture).expect("RR-0810: bulk Extended: Gateway agent relay implement pipeline v10");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0810_gateway_agent_relay_impl_extended::evaluate(&fixture[..end]).expect("RR-0810: stable prefix");
        assert!(partial.consumed <= end, "RR-0810: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0810: full prefix should match bulk checksum");
        }
    }
}
