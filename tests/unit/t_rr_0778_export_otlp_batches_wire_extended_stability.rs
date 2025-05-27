//! Integration test for `RR-0778` (stability).
//! Extended: Export OTLP batches wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0778_export_otlp_batches_wire_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0x15];
    let full = relayring::capabilities::rr_0778_export_otlp_batches_wire_extended::evaluate(fixture).expect("RR-0778: bulk Extended: Export OTLP batches wire planner v13");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0778_export_otlp_batches_wire_extended::evaluate(&fixture[..end]).expect("RR-0778: stable prefix");
        assert!(partial.consumed <= end, "RR-0778: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0778: full prefix should match bulk checksum");
        }
    }
}
