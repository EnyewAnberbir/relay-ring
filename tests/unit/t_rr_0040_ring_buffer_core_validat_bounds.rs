//! Integration test for `RR-0040` (bounds).
//! Ring buffer core validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0040_ring_buffer_core_validat_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0040_ring_buffer_core_validat::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0040_ring_buffer_core_validat::evaluate(fixture).expect("RR-0040 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
