//! Integration test for `RR-0299` (stability).
//! Export OTLP batches optimize registry v34 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0299_export_otlp_batches_opti_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x32];
    let full = relayring::capabilities::rr_0299_export_otlp_batches_opti::evaluate(fixture).expect("RR-0299: bulk Export OTLP batches optimize registry v34");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0299_export_otlp_batches_opti::evaluate(&fixture[..end]).expect("RR-0299: stable prefix");
        assert!(partial.consumed <= end, "RR-0299: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0299: full prefix should match bulk checksum");
        }
    }
}
