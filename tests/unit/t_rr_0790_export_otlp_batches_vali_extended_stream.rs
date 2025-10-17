//! Integration test for `RR-0790` (stream).
//! Extended: Export OTLP batches validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0790_export_otlp_batches_vali_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1f, 0x21];
    let direct = relayring::capabilities::rr_0790_export_otlp_batches_vali_extended::evaluate(fixture).expect("RR-0790: direct Extended: Export OTLP batches validate resolver v25");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0790_export_otlp_batches_vali_extended::evaluate(&copied).expect("RR-0790: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0790: stream path must consume input");
}
