//! Integration test for `RR-0312` (stability).
//! Gateway agent relay harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0312_gateway_agent_relay_hard_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3d, 0x3f];
    let full = relayring::capabilities::rr_0312_gateway_agent_relay_hard::evaluate(fixture).expect("RR-0312: bulk Gateway agent relay harden index v12");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0312_gateway_agent_relay_hard::evaluate(&fixture[..end]).expect("RR-0312: stable prefix");
        assert!(partial.consumed <= end, "RR-0312: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0312: full prefix should match bulk checksum");
        }
    }
}
