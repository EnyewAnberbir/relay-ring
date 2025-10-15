//! Integration test for `RR-0777` (stream).
//! Extended: Export OTLP batches harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0777_export_otlp_batches_hard_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12, 0x14];
    let direct = relayring::capabilities::rr_0777_export_otlp_batches_hard_extended::evaluate(fixture).expect("RR-0777: direct Extended: Export OTLP batches harden index v12");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0777_export_otlp_batches_hard_extended::evaluate(&copied).expect("RR-0777: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0777: stream path must consume input");
}
