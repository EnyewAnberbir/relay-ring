//! Integration test for `RR-0778` (stream).
//! Extended: Export OTLP batches wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0778_export_otlp_batches_wire_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0x15];
    let direct = relayring::capabilities::rr_0778_export_otlp_batches_wire_extended::evaluate(fixture).expect("RR-0778: direct Extended: Export OTLP batches wire planner v13");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0778_export_otlp_batches_wire_extended::evaluate(&copied).expect("RR-0778: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0778: stream path must consume input");
}
