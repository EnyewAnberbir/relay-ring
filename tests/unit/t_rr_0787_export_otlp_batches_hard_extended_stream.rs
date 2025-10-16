//! Integration test for `RR-0787` (stream).
//! Extended: Export OTLP batches harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0787_export_otlp_batches_hard_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1c, 0x1e];
    let direct = relayring::capabilities::rr_0787_export_otlp_batches_hard_extended::evaluate(fixture).expect("RR-0787: direct Extended: Export OTLP batches harden index v22");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0787_export_otlp_batches_hard_extended::evaluate(&copied).expect("RR-0787: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0787: stream path must consume input");
}
