//! Integration test for `RR-0280` (bounds).
//! Export OTLP batches validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0280_export_otlp_batches_vali_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0280_export_otlp_batches_vali::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0280_export_otlp_batches_vali::evaluate(fixture).expect("RR-0280 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
