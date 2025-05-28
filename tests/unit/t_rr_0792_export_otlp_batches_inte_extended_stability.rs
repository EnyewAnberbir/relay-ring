//! Integration test for `RR-0792` (stability).
//! Extended: Export OTLP batches integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0792_export_otlp_batches_inte_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x21, 0x23];
    let full = relayring::capabilities::rr_0792_export_otlp_batches_inte_extended::evaluate(fixture).expect("RR-0792: bulk Extended: Export OTLP batches integrate validator v27");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0792_export_otlp_batches_inte_extended::evaluate(&fixture[..end]).expect("RR-0792: stable prefix");
        assert!(partial.consumed <= end, "RR-0792: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0792: full prefix should match bulk checksum");
        }
    }
}
