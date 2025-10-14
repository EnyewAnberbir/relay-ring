//! Integration test for `RR-0771` (stream).
//! Extended: Export OTLP batches export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0771_export_otlp_batches_expo_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x0e];
    let direct = relayring::capabilities::rr_0771_export_otlp_batches_expo_extended::evaluate(fixture).expect("RR-0771: direct Extended: Export OTLP batches export adapter v6");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0771_export_otlp_batches_expo_extended::evaluate(&copied).expect("RR-0771: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0771: stream path must consume input");
}
