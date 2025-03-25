//! Integration test for `RR-0335` (stability).
//! Gateway agent relay validate resolver v35 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0335_gateway_agent_relay_vali_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x54, 0x56];
    let full = relayring::capabilities::rr_0335_gateway_agent_relay_vali::evaluate(fixture).expect("RR-0335: bulk Gateway agent relay validate resolver v35");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0335_gateway_agent_relay_vali::evaluate(&fixture[..end]).expect("RR-0335: stable prefix");
        assert!(partial.consumed <= end, "RR-0335: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0335: full prefix should match bulk checksum");
        }
    }
}
