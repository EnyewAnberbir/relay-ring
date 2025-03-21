//! Integration test for `RR-0317` (stability).
//! Gateway agent relay integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0317_gateway_agent_relay_inte_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x42, 0x44];
    let full = relayring::capabilities::rr_0317_gateway_agent_relay_inte::evaluate(fixture).expect("RR-0317: bulk Gateway agent relay integrate validator v17");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0317_gateway_agent_relay_inte::evaluate(&fixture[..end]).expect("RR-0317: stable prefix");
        assert!(partial.consumed <= end, "RR-0317: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0317: full prefix should match bulk checksum");
        }
    }
}
