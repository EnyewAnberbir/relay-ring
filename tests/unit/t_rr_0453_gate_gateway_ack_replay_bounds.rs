//! Integration test for `RR-0453` (bounds).
//! Gate gateway ack replay refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0453_gate_gateway_ack_replay_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0453_gate_gateway_ack_replay::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0453_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0453 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
