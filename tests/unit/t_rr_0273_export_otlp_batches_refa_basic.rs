//! Integration test for `RR-0273` (basic).
//! Export OTLP batches refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0273_export_otlp_batches_refa_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x16, 0x18];
    let first = relayring::capabilities::rr_0273_export_otlp_batches_refa::evaluate(fixture).expect("RR-0273: Export OTLP batches refactor mutator v8");
    let second = relayring::capabilities::rr_0273_export_otlp_batches_refa::evaluate(fixture).expect("RR-0273: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0273: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0273: stats visits every byte");
}
