//! Integration test for `RR-0774` (basic).
//! Extended: Export OTLP batches benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0774_export_otlp_batches_benc_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x11];
    let first = relayring::capabilities::rr_0774_export_otlp_batches_benc_extended::evaluate(fixture).expect("RR-0774: Extended: Export OTLP batches benchmark reporter v9");
    let second = relayring::capabilities::rr_0774_export_otlp_batches_benc_extended::evaluate(fixture).expect("RR-0774: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0774: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0774: scanner should emit domain hints");
}
