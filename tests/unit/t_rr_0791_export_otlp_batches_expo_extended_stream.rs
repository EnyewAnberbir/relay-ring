//! Integration test for `RR-0791` (stream).
//! Extended: Export OTLP batches export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0791_export_otlp_batches_expo_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x22];
    let direct = relayring::capabilities::rr_0791_export_otlp_batches_expo_extended::evaluate(fixture).expect("RR-0791: direct Extended: Export OTLP batches export adapter v26");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0791_export_otlp_batches_expo_extended::evaluate(&copied).expect("RR-0791: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0791: stream path must consume input");
}
