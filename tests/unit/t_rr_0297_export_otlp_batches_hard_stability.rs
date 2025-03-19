//! Integration test for `RR-0297` (stability).
//! Export OTLP batches harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0297_export_otlp_batches_hard_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2e, 0x30];
    let full = relayring::capabilities::rr_0297_export_otlp_batches_hard::evaluate(fixture).expect("RR-0297: bulk Export OTLP batches harden index v32");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0297_export_otlp_batches_hard::evaluate(&fixture[..end]).expect("RR-0297: stable prefix");
        assert!(partial.consumed <= end, "RR-0297: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0297: full prefix should match bulk checksum");
        }
    }
}
