//! Integration test for `RR-0772` (stream).
//! Extended: Export OTLP batches integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0772_export_otlp_batches_inte_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0d, 0x0f];
    let direct = relayring::capabilities::rr_0772_export_otlp_batches_inte_extended::evaluate(fixture).expect("RR-0772: direct Extended: Export OTLP batches integrate validator v7");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0772_export_otlp_batches_inte_extended::evaluate(&copied).expect("RR-0772: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0772: stream path must consume input");
}
