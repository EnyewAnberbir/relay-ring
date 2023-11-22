//! Integration test for `RR-0781` (basic).
//! Extended: Export OTLP batches export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0781_export_otlp_batches_expo_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x16, 0x18];
    let first = relayring::capabilities::rr_0781_export_otlp_batches_expo_extended::evaluate(fixture).expect("RR-0781: Extended: Export OTLP batches export adapter v16");
    let second = relayring::capabilities::rr_0781_export_otlp_batches_expo_extended::evaluate(fixture).expect("RR-0781: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0781: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0781: stats visits every byte");
}
