//! Integration test for `RR-0276` (stream).
//! Export OTLP batches extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0276_export_otlp_batches_exte_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x1b];
    let direct = relayring::capabilities::rr_0276_export_otlp_batches_exte::evaluate(fixture).expect("RR-0276: direct Export OTLP batches extend codec v11");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0276_export_otlp_batches_exte::evaluate(&copied).expect("RR-0276: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0276: stream path must consume input");
}
