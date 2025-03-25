//! Integration test for `RR-0333` (stability).
//! Gateway agent relay wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0333_gateway_agent_relay_wire_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x52, 0x54];
    let full = relayring::capabilities::rr_0333_gateway_agent_relay_wire::evaluate(fixture).expect("RR-0333: bulk Gateway agent relay wire planner v33");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0333_gateway_agent_relay_wire::evaluate(&fixture[..end]).expect("RR-0333: stable prefix");
        assert!(partial.consumed <= end, "RR-0333: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0333: full prefix should match bulk checksum");
        }
    }
}
