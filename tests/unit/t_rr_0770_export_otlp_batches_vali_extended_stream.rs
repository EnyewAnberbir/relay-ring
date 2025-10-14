//! Integration test for `RR-0770` (stream).
//! Extended: Export OTLP batches validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0770_export_otlp_batches_vali_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0b, 0x0d];
    let direct = relayring::capabilities::rr_0770_export_otlp_batches_vali_extended::evaluate(fixture).expect("RR-0770: direct Extended: Export OTLP batches validate resolver v5");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0770_export_otlp_batches_vali_extended::evaluate(&copied).expect("RR-0770: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0770: stream path must consume input");
}
