//! Integration test for `RR-0783` (stream).
//! Extended: Export OTLP batches refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0783_export_otlp_batches_refa_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x1a];
    let direct = relayring::capabilities::rr_0783_export_otlp_batches_refa_extended::evaluate(fixture).expect("RR-0783: direct Extended: Export OTLP batches refactor mutator v18");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0783_export_otlp_batches_refa_extended::evaluate(&copied).expect("RR-0783: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0783: stream path must consume input");
}
