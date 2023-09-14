//! Integration test for `RR-0276` (basic).
//! Export OTLP batches extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0276_export_otlp_batches_exte_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x1b];
    let first = relayring::capabilities::rr_0276_export_otlp_batches_exte::evaluate(fixture).expect("RR-0276: Export OTLP batches extend codec v11");
    let second = relayring::capabilities::rr_0276_export_otlp_batches_exte::evaluate(fixture).expect("RR-0276: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0276: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0276: stats visits every byte");
}
