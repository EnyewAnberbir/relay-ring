//! Integration test for `RR-0799` (stability).
//! Extended: Export OTLP batches optimize registry v34 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0799_export_otlp_batches_opti_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x28, 0x2a];
    let full = relayring::capabilities::rr_0799_export_otlp_batches_opti_extended::evaluate(fixture).expect("RR-0799: bulk Extended: Export OTLP batches optimize registry v34");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0799_export_otlp_batches_opti_extended::evaluate(&fixture[..end]).expect("RR-0799: stable prefix");
        assert!(partial.consumed <= end, "RR-0799: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0799: full prefix should match bulk checksum");
        }
    }
}
