//! Integration test for `RR-0299` (basic).
//! Export OTLP batches optimize registry v34 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0299_export_otlp_batches_opti_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x32];
    let first = relayring::capabilities::rr_0299_export_otlp_batches_opti::evaluate(fixture).expect("RR-0299: Export OTLP batches optimize registry v34");
    let second = relayring::capabilities::rr_0299_export_otlp_batches_opti::evaluate(fixture).expect("RR-0299: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0299: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0299: scanner should emit domain hints");
}
