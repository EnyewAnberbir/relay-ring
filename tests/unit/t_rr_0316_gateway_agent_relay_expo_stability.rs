//! Integration test for `RR-0316` (stability).
//! Gateway agent relay export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0316_gateway_agent_relay_expo_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x41, 0x43];
    let full = relayring::capabilities::rr_0316_gateway_agent_relay_expo::evaluate(fixture).expect("RR-0316: bulk Gateway agent relay export adapter v16");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0316_gateway_agent_relay_expo::evaluate(&fixture[..end]).expect("RR-0316: stable prefix");
        assert!(partial.consumed <= end, "RR-0316: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0316: full prefix should match bulk checksum");
        }
    }
}
