//! Integration test for `RR-0271` (stream).
//! Export OTLP batches export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0271_export_otlp_batches_expo_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x14, 0x16];
    let direct = relayring::capabilities::rr_0271_export_otlp_batches_expo::evaluate(fixture).expect("RR-0271: direct Export OTLP batches export adapter v6");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0271_export_otlp_batches_expo::evaluate(&copied).expect("RR-0271: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0271: stream path must consume input");
}
