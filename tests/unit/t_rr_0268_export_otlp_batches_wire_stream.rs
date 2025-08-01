//! Integration test for `RR-0268` (stream).
//! Export OTLP batches wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0268_export_otlp_batches_wire_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x13];
    let direct = relayring::capabilities::rr_0268_export_otlp_batches_wire::evaluate(fixture).expect("RR-0268: direct Export OTLP batches wire planner v3");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0268_export_otlp_batches_wire::evaluate(&copied).expect("RR-0268: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0268: stream path must consume input");
}
