//! Integration test for `RR-0281` (stream).
//! Export OTLP batches export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0281_export_otlp_batches_expo_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1e, 0x20];
    let direct = relayring::capabilities::rr_0281_export_otlp_batches_expo::evaluate(fixture).expect("RR-0281: direct Export OTLP batches export adapter v16");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0281_export_otlp_batches_expo::evaluate(&copied).expect("RR-0281: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0281: stream path must consume input");
}
