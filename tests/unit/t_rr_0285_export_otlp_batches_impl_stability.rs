//! Integration test for `RR-0285` (stability).
//! Export OTLP batches implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0285_export_otlp_batches_impl_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x22, 0x24];
    let full = relayring::capabilities::rr_0285_export_otlp_batches_impl::evaluate(fixture).expect("RR-0285: bulk Export OTLP batches implement pipeline v20");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0285_export_otlp_batches_impl::evaluate(&fixture[..end]).expect("RR-0285: stable prefix");
        assert!(partial.consumed <= end, "RR-0285: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0285: full prefix should match bulk checksum");
        }
    }
}
