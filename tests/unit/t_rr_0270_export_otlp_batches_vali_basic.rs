//! Integration test for `RR-0270` (basic).
//! Export OTLP batches validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0270_export_otlp_batches_vali_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0x15];
    let first = relayring::capabilities::rr_0270_export_otlp_batches_vali::evaluate(fixture).expect("RR-0270: Export OTLP batches validate resolver v5");
    let second = relayring::capabilities::rr_0270_export_otlp_batches_vali::evaluate(fixture).expect("RR-0270: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0270: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0270: scanner should emit domain hints");
}
