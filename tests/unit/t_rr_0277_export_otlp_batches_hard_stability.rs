//! Integration test for `RR-0277` (stability).
//! Export OTLP batches harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0277_export_otlp_batches_hard_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1a, 0x1c];
    let full = relayring::capabilities::rr_0277_export_otlp_batches_hard::evaluate(fixture).expect("RR-0277: bulk Export OTLP batches harden index v12");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0277_export_otlp_batches_hard::evaluate(&fixture[..end]).expect("RR-0277: stable prefix");
        assert!(partial.consumed <= end, "RR-0277: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0277: full prefix should match bulk checksum");
        }
    }
}
