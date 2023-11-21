//! Integration test for `RR-0772` (basic).
//! Extended: Export OTLP batches integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0772_export_otlp_batches_inte_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0d, 0x0f];
    let first = relayring::capabilities::rr_0772_export_otlp_batches_inte_extended::evaluate(fixture).expect("RR-0772: Extended: Export OTLP batches integrate validator v7");
    let second = relayring::capabilities::rr_0772_export_otlp_batches_inte_extended::evaluate(fixture).expect("RR-0772: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0772: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0772: scanner should emit domain hints");
}
