//! Integration test for `RR-0560` (bounds).
//! Extended: Ring buffer core validate resolver v35 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0560_ring_buffer_core_validat_extended_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0560_ring_buffer_core_validat_extended::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0560_ring_buffer_core_validat_extended::evaluate(fixture).expect("RR-0560 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
