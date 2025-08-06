//! Integration test for `RR-0299` (stream).
//! Export OTLP batches optimize registry v34 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0299_export_otlp_batches_opti_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x32];
    let direct = relayring::capabilities::rr_0299_export_otlp_batches_opti::evaluate(fixture).expect("RR-0299: direct Export OTLP batches optimize registry v34");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0299_export_otlp_batches_opti::evaluate(&copied).expect("RR-0299: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0299: stream path must consume input");
}
