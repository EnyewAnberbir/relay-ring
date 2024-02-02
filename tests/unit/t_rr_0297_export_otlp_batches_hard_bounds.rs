//! Integration test for `RR-0297` (bounds).
//! Export OTLP batches harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0297_export_otlp_batches_hard_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0297_export_otlp_batches_hard::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0297_export_otlp_batches_hard::evaluate(fixture).expect("RR-0297 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
