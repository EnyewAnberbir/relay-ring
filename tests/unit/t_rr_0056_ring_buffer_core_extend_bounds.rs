//! Integration test for `RR-0056` (bounds).
//! Ring buffer core extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0056_ring_buffer_core_extend_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0056_ring_buffer_core_extend::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0056_ring_buffer_core_extend::evaluate(fixture).expect("RR-0056 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
