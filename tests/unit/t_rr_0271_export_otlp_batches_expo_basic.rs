//! Integration test for `RR-0271` (basic).
//! Export OTLP batches export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0271_export_otlp_batches_expo_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x14, 0x16];
    let first = relayring::capabilities::rr_0271_export_otlp_batches_expo::evaluate(fixture).expect("RR-0271: Export OTLP batches export adapter v6");
    let second = relayring::capabilities::rr_0271_export_otlp_batches_expo::evaluate(fixture).expect("RR-0271: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0271: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0271: stats visits every byte");
}
