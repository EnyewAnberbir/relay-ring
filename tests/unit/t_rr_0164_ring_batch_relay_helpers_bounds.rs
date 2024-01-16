//! Integration test for `RR-0164` (bounds).
//! Ring batch relay helpers benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0164_ring_batch_relay_helpers_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0164_ring_batch_relay_helpers::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0164_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0164 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
