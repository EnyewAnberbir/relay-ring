//! Integration test for `RR-0802` (stability).
//! Extended: Gateway agent relay harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0802_gateway_agent_relay_hard_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2b, 0x2d];
    let full = relayring::capabilities::rr_0802_gateway_agent_relay_hard_extended::evaluate(fixture).expect("RR-0802: bulk Extended: Gateway agent relay harden index v2");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0802_gateway_agent_relay_hard_extended::evaluate(&fixture[..end]).expect("RR-0802: stable prefix");
        assert!(partial.consumed <= end, "RR-0802: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0802: full prefix should match bulk checksum");
        }
    }
}
