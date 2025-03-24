//! Integration test for `RR-0324` (stability).
//! Gateway agent relay optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0324_gateway_agent_relay_opti_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x49, 0x4b];
    let full = relayring::capabilities::rr_0324_gateway_agent_relay_opti::evaluate(fixture).expect("RR-0324: bulk Gateway agent relay optimize registry v24");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0324_gateway_agent_relay_opti::evaluate(&fixture[..end]).expect("RR-0324: stable prefix");
        assert!(partial.consumed <= end, "RR-0324: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0324: full prefix should match bulk checksum");
        }
    }
}
