//! Integration test for `RR-0302` (stability).
//! Gateway agent relay harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0302_gateway_agent_relay_hard_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x33, 0x35];
    let full = relayring::capabilities::rr_0302_gateway_agent_relay_hard::evaluate(fixture).expect("RR-0302: bulk Gateway agent relay harden index v2");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0302_gateway_agent_relay_hard::evaluate(&fixture[..end]).expect("RR-0302: stable prefix");
        assert!(partial.consumed <= end, "RR-0302: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0302: full prefix should match bulk checksum");
        }
    }
}
