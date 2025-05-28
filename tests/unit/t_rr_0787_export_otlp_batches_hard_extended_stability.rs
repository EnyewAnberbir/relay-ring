//! Integration test for `RR-0787` (stability).
//! Extended: Export OTLP batches harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0787_export_otlp_batches_hard_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1c, 0x1e];
    let full = relayring::capabilities::rr_0787_export_otlp_batches_hard_extended::evaluate(fixture).expect("RR-0787: bulk Extended: Export OTLP batches harden index v22");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0787_export_otlp_batches_hard_extended::evaluate(&fixture[..end]).expect("RR-0787: stable prefix");
        assert!(partial.consumed <= end, "RR-0787: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0787: full prefix should match bulk checksum");
        }
    }
}
