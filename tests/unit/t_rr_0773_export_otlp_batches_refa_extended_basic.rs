//! Integration test for `RR-0773` (basic).
//! Extended: Export OTLP batches refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0773_export_otlp_batches_refa_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0e, 0x10];
    let first = relayring::capabilities::rr_0773_export_otlp_batches_refa_extended::evaluate(fixture).expect("RR-0773: Extended: Export OTLP batches refactor mutator v8");
    let second = relayring::capabilities::rr_0773_export_otlp_batches_refa_extended::evaluate(fixture).expect("RR-0773: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0773: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0773: scanner should emit domain hints");
}
