//! Integration test for `RR-0831` (bounds).
//! Extended: Gateway agent relay extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0831_gateway_agent_relay_exte_extended_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0831_gateway_agent_relay_exte_extended::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0831_gateway_agent_relay_exte_extended::evaluate(fixture).expect("RR-0831 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
