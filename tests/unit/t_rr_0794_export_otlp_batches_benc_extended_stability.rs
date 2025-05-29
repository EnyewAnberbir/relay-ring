//! Integration test for `RR-0794` (stability).
//! Extended: Export OTLP batches benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0794_export_otlp_batches_benc_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x23, 0x25];
    let full = relayring::capabilities::rr_0794_export_otlp_batches_benc_extended::evaluate(fixture).expect("RR-0794: bulk Extended: Export OTLP batches benchmark reporter v29");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0794_export_otlp_batches_benc_extended::evaluate(&fixture[..end]).expect("RR-0794: stable prefix");
        assert!(partial.consumed <= end, "RR-0794: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0794: full prefix should match bulk checksum");
        }
    }
}
