//! Integration test for `RR-0295` (stability).
//! Export OTLP batches implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0295_export_otlp_batches_impl_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2c, 0x2e];
    let full = relayring::capabilities::rr_0295_export_otlp_batches_impl::evaluate(fixture).expect("RR-0295: bulk Export OTLP batches implement pipeline v30");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0295_export_otlp_batches_impl::evaluate(&fixture[..end]).expect("RR-0295: stable prefix");
        assert!(partial.consumed <= end, "RR-0295: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0295: full prefix should match bulk checksum");
        }
    }
}
