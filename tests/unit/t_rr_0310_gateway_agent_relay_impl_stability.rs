//! Integration test for `RR-0310` (stability).
//! Gateway agent relay implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0310_gateway_agent_relay_impl_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3b, 0x3d];
    let full = relayring::capabilities::rr_0310_gateway_agent_relay_impl::evaluate(fixture).expect("RR-0310: bulk Gateway agent relay implement pipeline v10");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0310_gateway_agent_relay_impl::evaluate(&fixture[..end]).expect("RR-0310: stable prefix");
        assert!(partial.consumed <= end, "RR-0310: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0310: full prefix should match bulk checksum");
        }
    }
}
