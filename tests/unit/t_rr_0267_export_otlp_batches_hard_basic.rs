//! Integration test for `RR-0267` (basic).
//! Export OTLP batches harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0267_export_otlp_batches_hard_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x12];
    let first = relayring::capabilities::rr_0267_export_otlp_batches_hard::evaluate(fixture).expect("RR-0267: Export OTLP batches harden index v2");
    let second = relayring::capabilities::rr_0267_export_otlp_batches_hard::evaluate(fixture).expect("RR-0267: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0267: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0267: stats visits every byte");
}
