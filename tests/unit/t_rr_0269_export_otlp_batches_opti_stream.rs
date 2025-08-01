//! Integration test for `RR-0269` (stream).
//! Export OTLP batches optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0269_export_otlp_batches_opti_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12, 0x14];
    let direct = relayring::capabilities::rr_0269_export_otlp_batches_opti::evaluate(fixture).expect("RR-0269: direct Export OTLP batches optimize registry v4");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0269_export_otlp_batches_opti::evaluate(&copied).expect("RR-0269: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0269: stream path must consume input");
}
