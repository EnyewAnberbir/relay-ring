//! Integration test for `RR-0287` (stream).
//! Export OTLP batches harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0287_export_otlp_batches_hard_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x24, 0x26];
    let direct = relayring::capabilities::rr_0287_export_otlp_batches_hard::evaluate(fixture).expect("RR-0287: direct Export OTLP batches harden index v22");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0287_export_otlp_batches_hard::evaluate(&copied).expect("RR-0287: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0287: stream path must consume input");
}
