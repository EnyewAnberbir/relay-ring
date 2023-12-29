//! Integration test for `RR-0031` (bounds).
//! Ring buffer core export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0031_ring_buffer_core_export_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0031_ring_buffer_core_export::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0031_ring_buffer_core_export::evaluate(fixture).expect("RR-0031 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
