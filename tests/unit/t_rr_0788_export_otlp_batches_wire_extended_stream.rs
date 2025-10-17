//! Integration test for `RR-0788` (stream).
//! Extended: Export OTLP batches wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0788_export_otlp_batches_wire_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1d, 0x1f];
    let direct = relayring::capabilities::rr_0788_export_otlp_batches_wire_extended::evaluate(fixture).expect("RR-0788: direct Extended: Export OTLP batches wire planner v23");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0788_export_otlp_batches_wire_extended::evaluate(&copied).expect("RR-0788: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0788: stream path must consume input");
}
