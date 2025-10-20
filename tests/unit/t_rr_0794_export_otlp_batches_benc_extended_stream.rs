//! Integration test for `RR-0794` (stream).
//! Extended: Export OTLP batches benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0794_export_otlp_batches_benc_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x23, 0x25];
    let direct = relayring::capabilities::rr_0794_export_otlp_batches_benc_extended::evaluate(fixture).expect("RR-0794: direct Extended: Export OTLP batches benchmark reporter v29");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0794_export_otlp_batches_benc_extended::evaluate(&copied).expect("RR-0794: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0794: stream path must consume input");
}
