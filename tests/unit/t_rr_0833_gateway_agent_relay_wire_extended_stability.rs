//! Integration test for `RR-0833` (stability).
//! Extended: Gateway agent relay wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0833_gateway_agent_relay_wire_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4a, 0x4c];
    let full = relayring::capabilities::rr_0833_gateway_agent_relay_wire_extended::evaluate(fixture).expect("RR-0833: bulk Extended: Gateway agent relay wire planner v33");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0833_gateway_agent_relay_wire_extended::evaluate(&fixture[..end]).expect("RR-0833: stable prefix");
        assert!(partial.consumed <= end, "RR-0833: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0833: full prefix should match bulk checksum");
        }
    }
}
