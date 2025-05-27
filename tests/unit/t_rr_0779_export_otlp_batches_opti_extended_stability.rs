//! Integration test for `RR-0779` (stability).
//! Extended: Export OTLP batches optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0779_export_otlp_batches_opti_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x14, 0x16];
    let full = relayring::capabilities::rr_0779_export_otlp_batches_opti_extended::evaluate(fixture).expect("RR-0779: bulk Extended: Export OTLP batches optimize registry v14");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0779_export_otlp_batches_opti_extended::evaluate(&fixture[..end]).expect("RR-0779: stable prefix");
        assert!(partial.consumed <= end, "RR-0779: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0779: full prefix should match bulk checksum");
        }
    }
}
