//! Integration test for `RR-0284` (stability).
//! Export OTLP batches benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0284_export_otlp_batches_benc_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x21, 0x23];
    let full = relayring::capabilities::rr_0284_export_otlp_batches_benc::evaluate(fixture).expect("RR-0284: bulk Export OTLP batches benchmark reporter v19");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0284_export_otlp_batches_benc::evaluate(&fixture[..end]).expect("RR-0284: stable prefix");
        assert!(partial.consumed <= end, "RR-0284: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0284: full prefix should match bulk checksum");
        }
    }
}
