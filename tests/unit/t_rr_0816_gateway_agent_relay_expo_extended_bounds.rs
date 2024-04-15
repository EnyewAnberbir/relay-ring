//! Integration test for `RR-0816` (bounds).
//! Extended: Gateway agent relay export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0816_gateway_agent_relay_expo_extended_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0816_gateway_agent_relay_expo_extended::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0816_gateway_agent_relay_expo_extended::evaluate(fixture).expect("RR-0816 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
