//! Integration test for `RR-0296` (stability).
//! Export OTLP batches extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0296_export_otlp_batches_exte_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2d, 0x2f];
    let full = relayring::capabilities::rr_0296_export_otlp_batches_exte::evaluate(fixture).expect("RR-0296: bulk Export OTLP batches extend codec v31");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0296_export_otlp_batches_exte::evaluate(&fixture[..end]).expect("RR-0296: stable prefix");
        assert!(partial.consumed <= end, "RR-0296: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0296: full prefix should match bulk checksum");
        }
    }
}
