//! Integration test for `RR-0785` (stream).
//! Extended: Export OTLP batches implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0785_export_otlp_batches_impl_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1a, 0x1c];
    let direct = relayring::capabilities::rr_0785_export_otlp_batches_impl_extended::evaluate(fixture).expect("RR-0785: direct Extended: Export OTLP batches implement pipeline v20");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0785_export_otlp_batches_impl_extended::evaluate(&copied).expect("RR-0785: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0785: stream path must consume input");
}
