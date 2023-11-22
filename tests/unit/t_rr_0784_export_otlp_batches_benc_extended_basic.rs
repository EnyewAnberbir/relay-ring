//! Integration test for `RR-0784` (basic).
//! Extended: Export OTLP batches benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0784_export_otlp_batches_benc_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x1b];
    let first = relayring::capabilities::rr_0784_export_otlp_batches_benc_extended::evaluate(fixture).expect("RR-0784: Extended: Export OTLP batches benchmark reporter v19");
    let second = relayring::capabilities::rr_0784_export_otlp_batches_benc_extended::evaluate(fixture).expect("RR-0784: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0784: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0784: scanner should emit domain hints");
}
