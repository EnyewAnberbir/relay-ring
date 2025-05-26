//! Integration test for `RR-0770` (stability).
//! Extended: Export OTLP batches validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0770_export_otlp_batches_vali_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0b, 0x0d];
    let full = relayring::capabilities::rr_0770_export_otlp_batches_vali_extended::evaluate(fixture).expect("RR-0770: bulk Extended: Export OTLP batches validate resolver v5");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0770_export_otlp_batches_vali_extended::evaluate(&fixture[..end]).expect("RR-0770: stable prefix");
        assert!(partial.consumed <= end, "RR-0770: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0770: full prefix should match bulk checksum");
        }
    }
}
