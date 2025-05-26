//! Integration test for `RR-0769` (stability).
//! Extended: Export OTLP batches optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0769_export_otlp_batches_opti_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0a, 0x0c];
    let full = relayring::capabilities::rr_0769_export_otlp_batches_opti_extended::evaluate(fixture).expect("RR-0769: bulk Extended: Export OTLP batches optimize registry v4");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0769_export_otlp_batches_opti_extended::evaluate(&fixture[..end]).expect("RR-0769: stable prefix");
        assert!(partial.consumed <= end, "RR-0769: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0769: full prefix should match bulk checksum");
        }
    }
}
