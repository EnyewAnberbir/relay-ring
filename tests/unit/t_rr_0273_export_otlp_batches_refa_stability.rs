//! Integration test for `RR-0273` (stability).
//! Export OTLP batches refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0273_export_otlp_batches_refa_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x16, 0x18];
    let full = relayring::capabilities::rr_0273_export_otlp_batches_refa::evaluate(fixture).expect("RR-0273: bulk Export OTLP batches refactor mutator v8");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0273_export_otlp_batches_refa::evaluate(&fixture[..end]).expect("RR-0273: stable prefix");
        assert!(partial.consumed <= end, "RR-0273: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0273: full prefix should match bulk checksum");
        }
    }
}
