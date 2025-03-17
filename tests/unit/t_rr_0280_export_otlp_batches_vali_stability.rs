//! Integration test for `RR-0280` (stability).
//! Export OTLP batches validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0280_export_otlp_batches_vali_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1d, 0x1f];
    let full = relayring::capabilities::rr_0280_export_otlp_batches_vali::evaluate(fixture).expect("RR-0280: bulk Export OTLP batches validate resolver v15");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0280_export_otlp_batches_vali::evaluate(&fixture[..end]).expect("RR-0280: stable prefix");
        assert!(partial.consumed <= end, "RR-0280: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0280: full prefix should match bulk checksum");
        }
    }
}
