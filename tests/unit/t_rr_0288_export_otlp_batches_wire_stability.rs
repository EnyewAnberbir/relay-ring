//! Integration test for `RR-0288` (stability).
//! Export OTLP batches wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0288_export_otlp_batches_wire_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x25, 0x27];
    let full = relayring::capabilities::rr_0288_export_otlp_batches_wire::evaluate(fixture).expect("RR-0288: bulk Export OTLP batches wire planner v23");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0288_export_otlp_batches_wire::evaluate(&fixture[..end]).expect("RR-0288: stable prefix");
        assert!(partial.consumed <= end, "RR-0288: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0288: full prefix should match bulk checksum");
        }
    }
}
