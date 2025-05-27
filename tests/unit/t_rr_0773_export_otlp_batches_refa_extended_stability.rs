//! Integration test for `RR-0773` (stability).
//! Extended: Export OTLP batches refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0773_export_otlp_batches_refa_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0e, 0x10];
    let full = relayring::capabilities::rr_0773_export_otlp_batches_refa_extended::evaluate(fixture).expect("RR-0773: bulk Extended: Export OTLP batches refactor mutator v8");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0773_export_otlp_batches_refa_extended::evaluate(&fixture[..end]).expect("RR-0773: stable prefix");
        assert!(partial.consumed <= end, "RR-0773: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0773: full prefix should match bulk checksum");
        }
    }
}
