//! Integration test for `RR-0283` (stream).
//! Export OTLP batches refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0283_export_otlp_batches_refa_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x22];
    let direct = relayring::capabilities::rr_0283_export_otlp_batches_refa::evaluate(fixture).expect("RR-0283: direct Export OTLP batches refactor mutator v18");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0283_export_otlp_batches_refa::evaluate(&copied).expect("RR-0283: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0283: stream path must consume input");
}
