//! Integration test for `RR-0307` (stability).
//! Gateway agent relay integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0307_gateway_agent_relay_inte_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x38, 0x3a];
    let full = relayring::capabilities::rr_0307_gateway_agent_relay_inte::evaluate(fixture).expect("RR-0307: bulk Gateway agent relay integrate validator v7");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0307_gateway_agent_relay_inte::evaluate(&fixture[..end]).expect("RR-0307: stable prefix");
        assert!(partial.consumed <= end, "RR-0307: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0307: full prefix should match bulk checksum");
        }
    }
}
