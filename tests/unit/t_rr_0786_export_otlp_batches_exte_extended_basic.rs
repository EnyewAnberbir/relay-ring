//! Integration test for `RR-0786` (basic).
//! Extended: Export OTLP batches extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0786_export_otlp_batches_exte_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1b, 0x1d];
    let first = relayring::capabilities::rr_0786_export_otlp_batches_exte_extended::evaluate(fixture).expect("RR-0786: Extended: Export OTLP batches extend codec v21");
    let second = relayring::capabilities::rr_0786_export_otlp_batches_exte_extended::evaluate(fixture).expect("RR-0786: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0786: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0786: scanner should emit domain hints");
}
