//! Integration test for `RR-0826` (stability).
//! Extended: Gateway agent relay export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0826_gateway_agent_relay_expo_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x43, 0x45];
    let full = relayring::capabilities::rr_0826_gateway_agent_relay_expo_extended::evaluate(fixture).expect("RR-0826: bulk Extended: Gateway agent relay export adapter v26");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0826_gateway_agent_relay_expo_extended::evaluate(&fixture[..end]).expect("RR-0826: stable prefix");
        assert!(partial.consumed <= end, "RR-0826: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0826: full prefix should match bulk checksum");
        }
    }
}
