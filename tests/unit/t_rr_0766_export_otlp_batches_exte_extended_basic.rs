//! Integration test for `RR-0766` (basic).
//! Extended: Export OTLP batches extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0766_export_otlp_batches_exte_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0x09];
    let first = relayring::capabilities::rr_0766_export_otlp_batches_exte_extended::evaluate(fixture).expect("RR-0766: Extended: Export OTLP batches extend codec v1");
    let second = relayring::capabilities::rr_0766_export_otlp_batches_exte_extended::evaluate(fixture).expect("RR-0766: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0766: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0766: scanner should emit domain hints");
}
