//! Integration test for `RR-0784` (stability).
//! Extended: Export OTLP batches benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0784_export_otlp_batches_benc_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x1b];
    let full = relayring::capabilities::rr_0784_export_otlp_batches_benc_extended::evaluate(fixture).expect("RR-0784: bulk Extended: Export OTLP batches benchmark reporter v19");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0784_export_otlp_batches_benc_extended::evaluate(&fixture[..end]).expect("RR-0784: stable prefix");
        assert!(partial.consumed <= end, "RR-0784: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0784: full prefix should match bulk checksum");
        }
    }
}
