//! Integration test for `RR-0291` (stream).
//! Export OTLP batches export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0291_export_otlp_batches_expo_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x28, 0x2a];
    let direct = relayring::capabilities::rr_0291_export_otlp_batches_expo::evaluate(fixture).expect("RR-0291: direct Export OTLP batches export adapter v26");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0291_export_otlp_batches_expo::evaluate(&copied).expect("RR-0291: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0291: stream path must consume input");
}
