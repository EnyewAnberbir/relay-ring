//! Integration test for `RR-0773` (stream).
//! Extended: Export OTLP batches refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0773_export_otlp_batches_refa_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0e, 0x10];
    let direct = relayring::capabilities::rr_0773_export_otlp_batches_refa_extended::evaluate(fixture).expect("RR-0773: direct Extended: Export OTLP batches refactor mutator v8");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0773_export_otlp_batches_refa_extended::evaluate(&copied).expect("RR-0773: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0773: stream path must consume input");
}
