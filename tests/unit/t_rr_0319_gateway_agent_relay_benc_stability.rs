//! Integration test for `RR-0319` (stability).
//! Gateway agent relay benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0319_gateway_agent_relay_benc_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x44, 0x46];
    let full = relayring::capabilities::rr_0319_gateway_agent_relay_benc::evaluate(fixture).expect("RR-0319: bulk Gateway agent relay benchmark reporter v19");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0319_gateway_agent_relay_benc::evaluate(&fixture[..end]).expect("RR-0319: stable prefix");
        assert!(partial.consumed <= end, "RR-0319: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0319: full prefix should match bulk checksum");
        }
    }
}
