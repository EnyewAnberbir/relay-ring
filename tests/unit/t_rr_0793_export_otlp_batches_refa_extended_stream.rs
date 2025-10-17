//! Integration test for `RR-0793` (stream).
//! Extended: Export OTLP batches refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0793_export_otlp_batches_refa_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x22, 0x24];
    let direct = relayring::capabilities::rr_0793_export_otlp_batches_refa_extended::evaluate(fixture).expect("RR-0793: direct Extended: Export OTLP batches refactor mutator v28");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0793_export_otlp_batches_refa_extended::evaluate(&copied).expect("RR-0793: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0793: stream path must consume input");
}
