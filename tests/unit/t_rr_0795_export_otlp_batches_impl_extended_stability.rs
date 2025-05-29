//! Integration test for `RR-0795` (stability).
//! Extended: Export OTLP batches implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0795_export_otlp_batches_impl_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x24, 0x26];
    let full = relayring::capabilities::rr_0795_export_otlp_batches_impl_extended::evaluate(fixture).expect("RR-0795: bulk Extended: Export OTLP batches implement pipeline v30");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0795_export_otlp_batches_impl_extended::evaluate(&fixture[..end]).expect("RR-0795: stable prefix");
        assert!(partial.consumed <= end, "RR-0795: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0795: full prefix should match bulk checksum");
        }
    }
}
