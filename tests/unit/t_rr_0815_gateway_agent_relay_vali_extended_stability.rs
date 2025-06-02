//! Integration test for `RR-0815` (stability).
//! Extended: Gateway agent relay validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0815_gateway_agent_relay_vali_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x38, 0x3a];
    let full = relayring::capabilities::rr_0815_gateway_agent_relay_vali_extended::evaluate(fixture).expect("RR-0815: bulk Extended: Gateway agent relay validate resolver v15");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0815_gateway_agent_relay_vali_extended::evaluate(&fixture[..end]).expect("RR-0815: stable prefix");
        assert!(partial.consumed <= end, "RR-0815: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0815: full prefix should match bulk checksum");
        }
    }
}
