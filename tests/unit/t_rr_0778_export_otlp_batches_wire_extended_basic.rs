//! Integration test for `RR-0778` (basic).
//! Extended: Export OTLP batches wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0778_export_otlp_batches_wire_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0x15];
    let first = relayring::capabilities::rr_0778_export_otlp_batches_wire_extended::evaluate(fixture).expect("RR-0778: Extended: Export OTLP batches wire planner v13");
    let second = relayring::capabilities::rr_0778_export_otlp_batches_wire_extended::evaluate(fixture).expect("RR-0778: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0778: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0778: stats visits every byte");
}
