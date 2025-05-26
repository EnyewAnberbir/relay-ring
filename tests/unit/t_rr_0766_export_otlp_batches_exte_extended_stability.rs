//! Integration test for `RR-0766` (stability).
//! Extended: Export OTLP batches extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0766_export_otlp_batches_exte_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0x09];
    let full = relayring::capabilities::rr_0766_export_otlp_batches_exte_extended::evaluate(fixture).expect("RR-0766: bulk Extended: Export OTLP batches extend codec v1");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0766_export_otlp_batches_exte_extended::evaluate(&fixture[..end]).expect("RR-0766: stable prefix");
        assert!(partial.consumed <= end, "RR-0766: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0766: full prefix should match bulk checksum");
        }
    }
}
