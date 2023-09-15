//! Integration test for `RR-0287` (basic).
//! Export OTLP batches harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0287_export_otlp_batches_hard_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x24, 0x26];
    let first = relayring::capabilities::rr_0287_export_otlp_batches_hard::evaluate(fixture).expect("RR-0287: Export OTLP batches harden index v22");
    let second = relayring::capabilities::rr_0287_export_otlp_batches_hard::evaluate(fixture).expect("RR-0287: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0287: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0287: stats visits every byte");
}
