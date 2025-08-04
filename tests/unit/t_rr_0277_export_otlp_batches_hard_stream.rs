//! Integration test for `RR-0277` (stream).
//! Export OTLP batches harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0277_export_otlp_batches_hard_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1a, 0x1c];
    let direct = relayring::capabilities::rr_0277_export_otlp_batches_hard::evaluate(fixture).expect("RR-0277: direct Export OTLP batches harden index v12");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0277_export_otlp_batches_hard::evaluate(&copied).expect("RR-0277: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0277: stream path must consume input");
}
