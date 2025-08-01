//! Integration test for `RR-0267` (stream).
//! Export OTLP batches harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0267_export_otlp_batches_hard_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x12];
    let direct = relayring::capabilities::rr_0267_export_otlp_batches_hard::evaluate(fixture).expect("RR-0267: direct Export OTLP batches harden index v2");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0267_export_otlp_batches_hard::evaluate(&copied).expect("RR-0267: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0267: stream path must consume input");
}
