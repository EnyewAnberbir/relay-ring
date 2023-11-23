//! Integration test for `RR-0792` (basic).
//! Extended: Export OTLP batches integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0792_export_otlp_batches_inte_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x21, 0x23];
    let first = relayring::capabilities::rr_0792_export_otlp_batches_inte_extended::evaluate(fixture).expect("RR-0792: Extended: Export OTLP batches integrate validator v27");
    let second = relayring::capabilities::rr_0792_export_otlp_batches_inte_extended::evaluate(fixture).expect("RR-0792: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0792: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0792: stats visits every byte");
}
