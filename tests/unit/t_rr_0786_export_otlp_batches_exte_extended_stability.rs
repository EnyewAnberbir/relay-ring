//! Integration test for `RR-0786` (stability).
//! Extended: Export OTLP batches extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0786_export_otlp_batches_exte_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1b, 0x1d];
    let full = relayring::capabilities::rr_0786_export_otlp_batches_exte_extended::evaluate(fixture).expect("RR-0786: bulk Extended: Export OTLP batches extend codec v21");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0786_export_otlp_batches_exte_extended::evaluate(&fixture[..end]).expect("RR-0786: stable prefix");
        assert!(partial.consumed <= end, "RR-0786: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0786: full prefix should match bulk checksum");
        }
    }
}
