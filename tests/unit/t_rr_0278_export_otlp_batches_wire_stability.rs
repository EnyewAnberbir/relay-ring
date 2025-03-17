//! Integration test for `RR-0278` (stability).
//! Export OTLP batches wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0278_export_otlp_batches_wire_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1b, 0x1d];
    let full = relayring::capabilities::rr_0278_export_otlp_batches_wire::evaluate(fixture).expect("RR-0278: bulk Export OTLP batches wire planner v13");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0278_export_otlp_batches_wire::evaluate(&fixture[..end]).expect("RR-0278: stable prefix");
        assert!(partial.consumed <= end, "RR-0278: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0278: full prefix should match bulk checksum");
        }
    }
}
