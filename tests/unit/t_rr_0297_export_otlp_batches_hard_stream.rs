//! Integration test for `RR-0297` (stream).
//! Export OTLP batches harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0297_export_otlp_batches_hard_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2e, 0x30];
    let direct = relayring::capabilities::rr_0297_export_otlp_batches_hard::evaluate(fixture).expect("RR-0297: direct Export OTLP batches harden index v32");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0297_export_otlp_batches_hard::evaluate(&copied).expect("RR-0297: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0297: stream path must consume input");
}
