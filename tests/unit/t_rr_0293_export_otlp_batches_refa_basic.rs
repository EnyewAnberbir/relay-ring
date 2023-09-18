//! Integration test for `RR-0293` (basic).
//! Export OTLP batches refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0293_export_otlp_batches_refa_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2a, 0x2c];
    let first = relayring::capabilities::rr_0293_export_otlp_batches_refa::evaluate(fixture).expect("RR-0293: Export OTLP batches refactor mutator v28");
    let second = relayring::capabilities::rr_0293_export_otlp_batches_refa::evaluate(fixture).expect("RR-0293: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0293: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0293: scanner should emit domain hints");
}
