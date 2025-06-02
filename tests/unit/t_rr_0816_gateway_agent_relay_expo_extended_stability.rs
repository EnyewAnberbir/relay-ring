//! Integration test for `RR-0816` (stability).
//! Extended: Gateway agent relay export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0816_gateway_agent_relay_expo_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x39, 0x3b];
    let full = relayring::capabilities::rr_0816_gateway_agent_relay_expo_extended::evaluate(fixture).expect("RR-0816: bulk Extended: Gateway agent relay export adapter v16");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0816_gateway_agent_relay_expo_extended::evaluate(&fixture[..end]).expect("RR-0816: stable prefix");
        assert!(partial.consumed <= end, "RR-0816: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0816: full prefix should match bulk checksum");
        }
    }
}
