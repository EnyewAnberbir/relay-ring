//! Integration test for `RR-0786` (stream).
//! Extended: Export OTLP batches extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0786_export_otlp_batches_exte_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1b, 0x1d];
    let direct = relayring::capabilities::rr_0786_export_otlp_batches_exte_extended::evaluate(fixture).expect("RR-0786: direct Extended: Export OTLP batches extend codec v21");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0786_export_otlp_batches_exte_extended::evaluate(&copied).expect("RR-0786: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0786: stream path must consume input");
}
