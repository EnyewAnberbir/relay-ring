//! Integration test for `RR-0306` (stability).
//! Gateway agent relay export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0306_gateway_agent_relay_expo_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x37, 0x39];
    let full = relayring::capabilities::rr_0306_gateway_agent_relay_expo::evaluate(fixture).expect("RR-0306: bulk Gateway agent relay export adapter v6");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0306_gateway_agent_relay_expo::evaluate(&fixture[..end]).expect("RR-0306: stable prefix");
        assert!(partial.consumed <= end, "RR-0306: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0306: full prefix should match bulk checksum");
        }
    }
}
