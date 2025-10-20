//! Integration test for `RR-0798` (stream).
//! Extended: Export OTLP batches wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0798_export_otlp_batches_wire_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x27, 0x29];
    let direct = relayring::capabilities::rr_0798_export_otlp_batches_wire_extended::evaluate(fixture).expect("RR-0798: direct Extended: Export OTLP batches wire planner v33");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0798_export_otlp_batches_wire_extended::evaluate(&copied).expect("RR-0798: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0798: stream path must consume input");
}
