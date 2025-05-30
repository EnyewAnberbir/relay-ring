//! Integration test for `RR-0813` (stability).
//! Extended: Gateway agent relay wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0813_gateway_agent_relay_wire_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x36, 0x38];
    let full = relayring::capabilities::rr_0813_gateway_agent_relay_wire_extended::evaluate(fixture).expect("RR-0813: bulk Extended: Gateway agent relay wire planner v13");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0813_gateway_agent_relay_wire_extended::evaluate(&fixture[..end]).expect("RR-0813: stable prefix");
        assert!(partial.consumed <= end, "RR-0813: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0813: full prefix should match bulk checksum");
        }
    }
}
