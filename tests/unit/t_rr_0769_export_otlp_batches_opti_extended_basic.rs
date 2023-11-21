//! Integration test for `RR-0769` (basic).
//! Extended: Export OTLP batches optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0769_export_otlp_batches_opti_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0a, 0x0c];
    let first = relayring::capabilities::rr_0769_export_otlp_batches_opti_extended::evaluate(fixture).expect("RR-0769: Extended: Export OTLP batches optimize registry v4");
    let second = relayring::capabilities::rr_0769_export_otlp_batches_opti_extended::evaluate(fixture).expect("RR-0769: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0769: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0769: stats visits every byte");
}
