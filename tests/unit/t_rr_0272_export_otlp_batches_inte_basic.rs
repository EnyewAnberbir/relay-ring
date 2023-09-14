//! Integration test for `RR-0272` (basic).
//! Export OTLP batches integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0272_export_otlp_batches_inte_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x15, 0x17];
    let first = relayring::capabilities::rr_0272_export_otlp_batches_inte::evaluate(fixture).expect("RR-0272: Export OTLP batches integrate validator v7");
    let second = relayring::capabilities::rr_0272_export_otlp_batches_inte::evaluate(fixture).expect("RR-0272: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0272: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0272: stats visits every byte");
}
