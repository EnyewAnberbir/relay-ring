//! Integration test for `RR-0771` (stability).
//! Extended: Export OTLP batches export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0771_export_otlp_batches_expo_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x0e];
    let full = relayring::capabilities::rr_0771_export_otlp_batches_expo_extended::evaluate(fixture).expect("RR-0771: bulk Extended: Export OTLP batches export adapter v6");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0771_export_otlp_batches_expo_extended::evaluate(&fixture[..end]).expect("RR-0771: stable prefix");
        assert!(partial.consumed <= end, "RR-0771: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0771: full prefix should match bulk checksum");
        }
    }
}
