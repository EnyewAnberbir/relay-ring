//! Integration test for `RR-0289` (basic).
//! Export OTLP batches optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0289_export_otlp_batches_opti_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x26, 0x28];
    let first = relayring::capabilities::rr_0289_export_otlp_batches_opti::evaluate(fixture).expect("RR-0289: Export OTLP batches optimize registry v24");
    let second = relayring::capabilities::rr_0289_export_otlp_batches_opti::evaluate(fixture).expect("RR-0289: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0289: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0289: scanner should emit domain hints");
}
