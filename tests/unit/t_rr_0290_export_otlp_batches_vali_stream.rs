//! Integration test for `RR-0290` (stream).
//! Export OTLP batches validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0290_export_otlp_batches_vali_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x27, 0x29];
    let direct = relayring::capabilities::rr_0290_export_otlp_batches_vali::evaluate(fixture).expect("RR-0290: direct Export OTLP batches validate resolver v25");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0290_export_otlp_batches_vali::evaluate(&copied).expect("RR-0290: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0290: stream path must consume input");
}
