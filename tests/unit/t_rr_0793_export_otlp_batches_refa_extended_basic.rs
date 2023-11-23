//! Integration test for `RR-0793` (basic).
//! Extended: Export OTLP batches refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0793_export_otlp_batches_refa_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x22, 0x24];
    let first = relayring::capabilities::rr_0793_export_otlp_batches_refa_extended::evaluate(fixture).expect("RR-0793: Extended: Export OTLP batches refactor mutator v28");
    let second = relayring::capabilities::rr_0793_export_otlp_batches_refa_extended::evaluate(fixture).expect("RR-0793: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0793: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0793: scanner should emit domain hints");
}
