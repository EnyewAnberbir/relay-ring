//! Integration test for `RR-0332` (stability).
//! Gateway agent relay harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0332_gateway_agent_relay_hard_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x51, 0x53];
    let full = relayring::capabilities::rr_0332_gateway_agent_relay_hard::evaluate(fixture).expect("RR-0332: bulk Gateway agent relay harden index v32");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0332_gateway_agent_relay_hard::evaluate(&fixture[..end]).expect("RR-0332: stable prefix");
        assert!(partial.consumed <= end, "RR-0332: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0332: full prefix should match bulk checksum");
        }
    }
}
