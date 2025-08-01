//! Integration test for `RR-0275` (stream).
//! Export OTLP batches implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0275_export_otlp_batches_impl_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x1a];
    let direct = relayring::capabilities::rr_0275_export_otlp_batches_impl::evaluate(fixture).expect("RR-0275: direct Export OTLP batches implement pipeline v10");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0275_export_otlp_batches_impl::evaluate(&copied).expect("RR-0275: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0275: stream path must consume input");
}
