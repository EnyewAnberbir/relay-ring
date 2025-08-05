//! Integration test for `RR-0294` (stream).
//! Export OTLP batches benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0294_export_otlp_batches_benc_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2b, 0x2d];
    let direct = relayring::capabilities::rr_0294_export_otlp_batches_benc::evaluate(fixture).expect("RR-0294: direct Export OTLP batches benchmark reporter v29");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0294_export_otlp_batches_benc::evaluate(&copied).expect("RR-0294: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0294: stream path must consume input");
}
