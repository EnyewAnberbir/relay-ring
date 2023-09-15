//! Integration test for `RR-0288` (basic).
//! Export OTLP batches wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0288_export_otlp_batches_wire_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x25, 0x27];
    let first = relayring::capabilities::rr_0288_export_otlp_batches_wire::evaluate(fixture).expect("RR-0288: Export OTLP batches wire planner v23");
    let second = relayring::capabilities::rr_0288_export_otlp_batches_wire::evaluate(fixture).expect("RR-0288: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0288: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0288: scanner should emit domain hints");
}
