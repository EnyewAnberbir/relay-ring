//! Integration test for `RR-0782` (basic).
//! Extended: Export OTLP batches integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0782_export_otlp_batches_inte_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x17, 0x19];
    let first = relayring::capabilities::rr_0782_export_otlp_batches_inte_extended::evaluate(fixture).expect("RR-0782: Extended: Export OTLP batches integrate validator v17");
    let second = relayring::capabilities::rr_0782_export_otlp_batches_inte_extended::evaluate(fixture).expect("RR-0782: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0782: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0782: stats visits every byte");
}
