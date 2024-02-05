//! Integration test for `RR-0309` (bounds).
//! Gateway agent relay benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0309_gateway_agent_relay_benc_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0309_gateway_agent_relay_benc::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0309_gateway_agent_relay_benc::evaluate(fixture).expect("RR-0309 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
