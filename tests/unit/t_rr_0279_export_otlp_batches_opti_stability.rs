//! Integration test for `RR-0279` (stability).
//! Export OTLP batches optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0279_export_otlp_batches_opti_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1c, 0x1e];
    let full = relayring::capabilities::rr_0279_export_otlp_batches_opti::evaluate(fixture).expect("RR-0279: bulk Export OTLP batches optimize registry v14");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0279_export_otlp_batches_opti::evaluate(&fixture[..end]).expect("RR-0279: stable prefix");
        assert!(partial.consumed <= end, "RR-0279: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0279: full prefix should match bulk checksum");
        }
    }
}
