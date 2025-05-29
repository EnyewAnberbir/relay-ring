//! Integration test for `RR-0796` (stability).
//! Extended: Export OTLP batches extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0796_export_otlp_batches_exte_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x25, 0x27];
    let full = relayring::capabilities::rr_0796_export_otlp_batches_exte_extended::evaluate(fixture).expect("RR-0796: bulk Extended: Export OTLP batches extend codec v31");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0796_export_otlp_batches_exte_extended::evaluate(&fixture[..end]).expect("RR-0796: stable prefix");
        assert!(partial.consumed <= end, "RR-0796: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0796: full prefix should match bulk checksum");
        }
    }
}
