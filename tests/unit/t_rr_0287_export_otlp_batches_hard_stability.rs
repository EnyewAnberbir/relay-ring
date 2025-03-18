//! Integration test for `RR-0287` (stability).
//! Export OTLP batches harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0287_export_otlp_batches_hard_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x24, 0x26];
    let full = relayring::capabilities::rr_0287_export_otlp_batches_hard::evaluate(fixture).expect("RR-0287: bulk Export OTLP batches harden index v22");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0287_export_otlp_batches_hard::evaluate(&fixture[..end]).expect("RR-0287: stable prefix");
        assert!(partial.consumed <= end, "RR-0287: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0287: full prefix should match bulk checksum");
        }
    }
}
