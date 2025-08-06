//! Integration test for `RR-0298` (stream).
//! Export OTLP batches wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0298_export_otlp_batches_wire_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2f, 0x31];
    let direct = relayring::capabilities::rr_0298_export_otlp_batches_wire::evaluate(fixture).expect("RR-0298: direct Export OTLP batches wire planner v33");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0298_export_otlp_batches_wire::evaluate(&copied).expect("RR-0298: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0298: stream path must consume input");
}
