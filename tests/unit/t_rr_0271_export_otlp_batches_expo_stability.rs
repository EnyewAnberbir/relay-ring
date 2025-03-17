//! Integration test for `RR-0271` (stability).
//! Export OTLP batches export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0271_export_otlp_batches_expo_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x14, 0x16];
    let full = relayring::capabilities::rr_0271_export_otlp_batches_expo::evaluate(fixture).expect("RR-0271: bulk Export OTLP batches export adapter v6");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0271_export_otlp_batches_expo::evaluate(&fixture[..end]).expect("RR-0271: stable prefix");
        assert!(partial.consumed <= end, "RR-0271: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0271: full prefix should match bulk checksum");
        }
    }
}
