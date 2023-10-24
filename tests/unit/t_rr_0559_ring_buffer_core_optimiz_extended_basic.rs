//! Integration test for `RR-0559` (basic).
//! Extended: Ring buffer core optimize registry v34 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0559_ring_buffer_core_optimiz_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x36, 0x38];
    let first = relayring::capabilities::rr_0559_ring_buffer_core_optimiz_extended::evaluate(fixture).expect("RR-0559: Extended: Ring buffer core optimize registry v34");
    let second = relayring::capabilities::rr_0559_ring_buffer_core_optimiz_extended::evaluate(fixture).expect("RR-0559: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0559: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0559: stats visits every byte");
}
