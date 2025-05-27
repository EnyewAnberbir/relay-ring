//! Integration test for `RR-0777` (stability).
//! Extended: Export OTLP batches harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0777_export_otlp_batches_hard_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12, 0x14];
    let full = relayring::capabilities::rr_0777_export_otlp_batches_hard_extended::evaluate(fixture).expect("RR-0777: bulk Extended: Export OTLP batches harden index v12");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0777_export_otlp_batches_hard_extended::evaluate(&fixture[..end]).expect("RR-0777: stable prefix");
        assert!(partial.consumed <= end, "RR-0777: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0777: full prefix should match bulk checksum");
        }
    }
}
