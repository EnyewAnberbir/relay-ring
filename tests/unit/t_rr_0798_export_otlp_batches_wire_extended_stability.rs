//! Integration test for `RR-0798` (stability).
//! Extended: Export OTLP batches wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0798_export_otlp_batches_wire_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x27, 0x29];
    let full = relayring::capabilities::rr_0798_export_otlp_batches_wire_extended::evaluate(fixture).expect("RR-0798: bulk Extended: Export OTLP batches wire planner v33");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0798_export_otlp_batches_wire_extended::evaluate(&fixture[..end]).expect("RR-0798: stable prefix");
        assert!(partial.consumed <= end, "RR-0798: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0798: full prefix should match bulk checksum");
        }
    }
}
