//! Integration test for `RR-0767` (stream).
//! Extended: Export OTLP batches harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0767_export_otlp_batches_hard_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x0a];
    let direct = relayring::capabilities::rr_0767_export_otlp_batches_hard_extended::evaluate(fixture).expect("RR-0767: direct Extended: Export OTLP batches harden index v2");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0767_export_otlp_batches_hard_extended::evaluate(&copied).expect("RR-0767: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0767: stream path must consume input");
}
