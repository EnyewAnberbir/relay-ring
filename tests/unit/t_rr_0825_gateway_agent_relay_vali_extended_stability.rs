//! Integration test for `RR-0825` (stability).
//! Extended: Gateway agent relay validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0825_gateway_agent_relay_vali_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x42, 0x44];
    let full = relayring::capabilities::rr_0825_gateway_agent_relay_vali_extended::evaluate(fixture).expect("RR-0825: bulk Extended: Gateway agent relay validate resolver v25");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0825_gateway_agent_relay_vali_extended::evaluate(&fixture[..end]).expect("RR-0825: stable prefix");
        assert!(partial.consumed <= end, "RR-0825: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0825: full prefix should match bulk checksum");
        }
    }
}
