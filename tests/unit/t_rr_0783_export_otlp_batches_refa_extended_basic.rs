//! Integration test for `RR-0783` (basic).
//! Extended: Export OTLP batches refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0783_export_otlp_batches_refa_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x1a];
    let first = relayring::capabilities::rr_0783_export_otlp_batches_refa_extended::evaluate(fixture).expect("RR-0783: Extended: Export OTLP batches refactor mutator v18");
    let second = relayring::capabilities::rr_0783_export_otlp_batches_refa_extended::evaluate(fixture).expect("RR-0783: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0783: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0783: window consumes the whole buffer");
}
