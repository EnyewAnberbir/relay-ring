//! Integration test for `RR-0322` (stability).
//! Gateway agent relay harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0322_gateway_agent_relay_hard_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x47, 0x49];
    let full = relayring::capabilities::rr_0322_gateway_agent_relay_hard::evaluate(fixture).expect("RR-0322: bulk Gateway agent relay harden index v22");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0322_gateway_agent_relay_hard::evaluate(&fixture[..end]).expect("RR-0322: stable prefix");
        assert!(partial.consumed <= end, "RR-0322: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0322: full prefix should match bulk checksum");
        }
    }
}
