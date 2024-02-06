//! Integration test for `RR-0325` (bounds).
//! Gateway agent relay validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0325_gateway_agent_relay_vali_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0325_gateway_agent_relay_vali::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0325_gateway_agent_relay_vali::evaluate(fixture).expect("RR-0325 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
