//! Integration test for `RR-0834` (stability).
//! Extended: Gateway agent relay optimize registry v34 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0834_gateway_agent_relay_opti_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4b, 0x4d];
    let full = relayring::capabilities::rr_0834_gateway_agent_relay_opti_extended::evaluate(fixture).expect("RR-0834: bulk Extended: Gateway agent relay optimize registry v34");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0834_gateway_agent_relay_opti_extended::evaluate(&fixture[..end]).expect("RR-0834: stable prefix");
        assert!(partial.consumed <= end, "RR-0834: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0834: full prefix should match bulk checksum");
        }
    }
}
