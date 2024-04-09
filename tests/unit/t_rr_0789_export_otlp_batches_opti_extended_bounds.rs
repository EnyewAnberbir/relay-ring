//! Integration test for `RR-0789` (bounds).
//! Extended: Export OTLP batches optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0789_export_otlp_batches_opti_extended_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0789_export_otlp_batches_opti_extended::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0789_export_otlp_batches_opti_extended::evaluate(fixture).expect("RR-0789 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
