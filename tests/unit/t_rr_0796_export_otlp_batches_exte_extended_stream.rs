//! Integration test for `RR-0796` (stream).
//! Extended: Export OTLP batches extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0796_export_otlp_batches_exte_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x25, 0x27];
    let direct = relayring::capabilities::rr_0796_export_otlp_batches_exte_extended::evaluate(fixture).expect("RR-0796: direct Extended: Export OTLP batches extend codec v31");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0796_export_otlp_batches_exte_extended::evaluate(&copied).expect("RR-0796: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0796: stream path must consume input");
}
