//! Integration test for `RR-0293` (stream).
//! Export OTLP batches refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0293_export_otlp_batches_refa_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2a, 0x2c];
    let direct = relayring::capabilities::rr_0293_export_otlp_batches_refa::evaluate(fixture).expect("RR-0293: direct Export OTLP batches refactor mutator v28");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0293_export_otlp_batches_refa::evaluate(&copied).expect("RR-0293: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0293: stream path must consume input");
}
