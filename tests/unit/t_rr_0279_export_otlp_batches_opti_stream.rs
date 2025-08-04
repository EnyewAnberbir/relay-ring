//! Integration test for `RR-0279` (stream).
//! Export OTLP batches optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0279_export_otlp_batches_opti_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1c, 0x1e];
    let direct = relayring::capabilities::rr_0279_export_otlp_batches_opti::evaluate(fixture).expect("RR-0279: direct Export OTLP batches optimize registry v14");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0279_export_otlp_batches_opti::evaluate(&copied).expect("RR-0279: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0279: stream path must consume input");
}
