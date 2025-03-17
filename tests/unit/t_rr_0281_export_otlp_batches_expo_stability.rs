//! Integration test for `RR-0281` (stability).
//! Export OTLP batches export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0281_export_otlp_batches_expo_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1e, 0x20];
    let full = relayring::capabilities::rr_0281_export_otlp_batches_expo::evaluate(fixture).expect("RR-0281: bulk Export OTLP batches export adapter v16");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0281_export_otlp_batches_expo::evaluate(&fixture[..end]).expect("RR-0281: stable prefix");
        assert!(partial.consumed <= end, "RR-0281: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0281: full prefix should match bulk checksum");
        }
    }
}
