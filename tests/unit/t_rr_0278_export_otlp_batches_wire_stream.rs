//! Integration test for `RR-0278` (stream).
//! Export OTLP batches wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0278_export_otlp_batches_wire_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1b, 0x1d];
    let direct = relayring::capabilities::rr_0278_export_otlp_batches_wire::evaluate(fixture).expect("RR-0278: direct Export OTLP batches wire planner v13");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0278_export_otlp_batches_wire::evaluate(&copied).expect("RR-0278: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0278: stream path must consume input");
}
