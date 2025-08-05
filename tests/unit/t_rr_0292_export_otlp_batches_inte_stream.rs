//! Integration test for `RR-0292` (stream).
//! Export OTLP batches integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0292_export_otlp_batches_inte_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x2b];
    let direct = relayring::capabilities::rr_0292_export_otlp_batches_inte::evaluate(fixture).expect("RR-0292: direct Export OTLP batches integrate validator v27");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0292_export_otlp_batches_inte::evaluate(&copied).expect("RR-0292: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0292: stream path must consume input");
}
