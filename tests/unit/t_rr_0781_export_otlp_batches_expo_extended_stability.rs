//! Integration test for `RR-0781` (stability).
//! Extended: Export OTLP batches export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0781_export_otlp_batches_expo_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x16, 0x18];
    let full = relayring::capabilities::rr_0781_export_otlp_batches_expo_extended::evaluate(fixture).expect("RR-0781: bulk Extended: Export OTLP batches export adapter v16");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0781_export_otlp_batches_expo_extended::evaluate(&fixture[..end]).expect("RR-0781: stable prefix");
        assert!(partial.consumed <= end, "RR-0781: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0781: full prefix should match bulk checksum");
        }
    }
}
