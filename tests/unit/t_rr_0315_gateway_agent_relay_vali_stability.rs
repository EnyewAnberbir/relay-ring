//! Integration test for `RR-0315` (stability).
//! Gateway agent relay validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0315_gateway_agent_relay_vali_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x42];
    let full = relayring::capabilities::rr_0315_gateway_agent_relay_vali::evaluate(fixture).expect("RR-0315: bulk Gateway agent relay validate resolver v15");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0315_gateway_agent_relay_vali::evaluate(&fixture[..end]).expect("RR-0315: stable prefix");
        assert!(partial.consumed <= end, "RR-0315: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0315: full prefix should match bulk checksum");
        }
    }
}
