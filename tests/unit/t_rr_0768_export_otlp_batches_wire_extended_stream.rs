//! Integration test for `RR-0768` (stream).
//! Extended: Export OTLP batches wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0768_export_otlp_batches_wire_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x0b];
    let direct = relayring::capabilities::rr_0768_export_otlp_batches_wire_extended::evaluate(fixture).expect("RR-0768: direct Extended: Export OTLP batches wire planner v3");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0768_export_otlp_batches_wire_extended::evaluate(&copied).expect("RR-0768: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0768: stream path must consume input");
}
