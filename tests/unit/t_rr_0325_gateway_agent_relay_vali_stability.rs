//! Integration test for `RR-0325` (stability).
//! Gateway agent relay validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0325_gateway_agent_relay_vali_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4a, 0x4c];
    let full = relayring::capabilities::rr_0325_gateway_agent_relay_vali::evaluate(fixture).expect("RR-0325: bulk Gateway agent relay validate resolver v25");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0325_gateway_agent_relay_vali::evaluate(&fixture[..end]).expect("RR-0325: stable prefix");
        assert!(partial.consumed <= end, "RR-0325: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0325: full prefix should match bulk checksum");
        }
    }
}
