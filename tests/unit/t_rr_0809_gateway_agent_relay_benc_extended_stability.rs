//! Integration test for `RR-0809` (stability).
//! Extended: Gateway agent relay benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0809_gateway_agent_relay_benc_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x32, 0x34];
    let full = relayring::capabilities::rr_0809_gateway_agent_relay_benc_extended::evaluate(fixture).expect("RR-0809: bulk Extended: Gateway agent relay benchmark reporter v9");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0809_gateway_agent_relay_benc_extended::evaluate(&fixture[..end]).expect("RR-0809: stable prefix");
        assert!(partial.consumed <= end, "RR-0809: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0809: full prefix should match bulk checksum");
        }
    }
}
