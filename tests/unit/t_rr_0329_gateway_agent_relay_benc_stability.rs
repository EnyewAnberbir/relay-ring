//! Integration test for `RR-0329` (stability).
//! Gateway agent relay benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0329_gateway_agent_relay_benc_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4e, 0x50];
    let full = relayring::capabilities::rr_0329_gateway_agent_relay_benc::evaluate(fixture).expect("RR-0329: bulk Gateway agent relay benchmark reporter v29");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0329_gateway_agent_relay_benc::evaluate(&fixture[..end]).expect("RR-0329: stable prefix");
        assert!(partial.consumed <= end, "RR-0329: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0329: full prefix should match bulk checksum");
        }
    }
}
