//! Integration test for `RR-0796` (basic).
//! Extended: Export OTLP batches extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0796_export_otlp_batches_exte_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x25, 0x27];
    let first = relayring::capabilities::rr_0796_export_otlp_batches_exte_extended::evaluate(fixture).expect("RR-0796: Extended: Export OTLP batches extend codec v31");
    let second = relayring::capabilities::rr_0796_export_otlp_batches_exte_extended::evaluate(fixture).expect("RR-0796: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0796: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0796: window consumes the whole buffer");
}
