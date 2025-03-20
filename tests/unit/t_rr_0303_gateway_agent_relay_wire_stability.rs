//! Integration test for `RR-0303` (stability).
//! Gateway agent relay wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0303_gateway_agent_relay_wire_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x34, 0x36];
    let full = relayring::capabilities::rr_0303_gateway_agent_relay_wire::evaluate(fixture).expect("RR-0303: bulk Gateway agent relay wire planner v3");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0303_gateway_agent_relay_wire::evaluate(&fixture[..end]).expect("RR-0303: stable prefix");
        assert!(partial.consumed <= end, "RR-0303: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0303: full prefix should match bulk checksum");
        }
    }
}
