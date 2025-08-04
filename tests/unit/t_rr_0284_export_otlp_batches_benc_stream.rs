//! Integration test for `RR-0284` (stream).
//! Export OTLP batches benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0284_export_otlp_batches_benc_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x21, 0x23];
    let direct = relayring::capabilities::rr_0284_export_otlp_batches_benc::evaluate(fixture).expect("RR-0284: direct Export OTLP batches benchmark reporter v19");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0284_export_otlp_batches_benc::evaluate(&copied).expect("RR-0284: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0284: stream path must consume input");
}
