//! Integration test for `RR-0821` (stability).
//! Extended: Gateway agent relay extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0821_gateway_agent_relay_exte_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3e, 0x40];
    let full = relayring::capabilities::rr_0821_gateway_agent_relay_exte_extended::evaluate(fixture).expect("RR-0821: bulk Extended: Gateway agent relay extend codec v21");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0821_gateway_agent_relay_exte_extended::evaluate(&fixture[..end]).expect("RR-0821: stable prefix");
        assert!(partial.consumed <= end, "RR-0821: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0821: full prefix should match bulk checksum");
        }
    }
}
