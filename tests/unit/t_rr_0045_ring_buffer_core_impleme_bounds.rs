//! Integration test for `RR-0045` (bounds).
//! Ring buffer core implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0045_ring_buffer_core_impleme_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0045_ring_buffer_core_impleme::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0045_ring_buffer_core_impleme::evaluate(fixture).expect("RR-0045 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
