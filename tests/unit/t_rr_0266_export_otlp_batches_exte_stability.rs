//! Integration test for `RR-0266` (stability).
//! Export OTLP batches extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0266_export_otlp_batches_exte_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x11];
    let full = relayring::capabilities::rr_0266_export_otlp_batches_exte::evaluate(fixture).expect("RR-0266: bulk Export OTLP batches extend codec v1");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0266_export_otlp_batches_exte::evaluate(&fixture[..end]).expect("RR-0266: stable prefix");
        assert!(partial.consumed <= end, "RR-0266: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0266: full prefix should match bulk checksum");
        }
    }
}
