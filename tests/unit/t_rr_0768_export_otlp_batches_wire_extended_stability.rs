//! Integration test for `RR-0768` (stability).
//! Extended: Export OTLP batches wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0768_export_otlp_batches_wire_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x0b];
    let full = relayring::capabilities::rr_0768_export_otlp_batches_wire_extended::evaluate(fixture).expect("RR-0768: bulk Extended: Export OTLP batches wire planner v3");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0768_export_otlp_batches_wire_extended::evaluate(&fixture[..end]).expect("RR-0768: stable prefix");
        assert!(partial.consumed <= end, "RR-0768: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0768: full prefix should match bulk checksum");
        }
    }
}
