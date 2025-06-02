//! Integration test for `RR-0822` (stability).
//! Extended: Gateway agent relay harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0822_gateway_agent_relay_hard_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0x41];
    let full = relayring::capabilities::rr_0822_gateway_agent_relay_hard_extended::evaluate(fixture).expect("RR-0822: bulk Extended: Gateway agent relay harden index v22");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0822_gateway_agent_relay_hard_extended::evaluate(&fixture[..end]).expect("RR-0822: stable prefix");
        assert!(partial.consumed <= end, "RR-0822: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0822: full prefix should match bulk checksum");
        }
    }
}
