//! Integration test for `RR-0330` (stability).
//! Gateway agent relay implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0330_gateway_agent_relay_impl_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4f, 0x51];
    let full = relayring::capabilities::rr_0330_gateway_agent_relay_impl::evaluate(fixture).expect("RR-0330: bulk Gateway agent relay implement pipeline v30");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0330_gateway_agent_relay_impl::evaluate(&fixture[..end]).expect("RR-0330: stable prefix");
        assert!(partial.consumed <= end, "RR-0330: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0330: full prefix should match bulk checksum");
        }
    }
}
