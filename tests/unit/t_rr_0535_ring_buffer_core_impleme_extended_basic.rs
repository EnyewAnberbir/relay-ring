//! Integration test for `RR-0535` (basic).
//! Extended: Ring buffer core implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0535_ring_buffer_core_impleme_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1e, 0x20];
    let first = relayring::capabilities::rr_0535_ring_buffer_core_impleme_extended::evaluate(fixture).expect("RR-0535: Extended: Ring buffer core implement pipeline v10");
    let second = relayring::capabilities::rr_0535_ring_buffer_core_impleme_extended::evaluate(fixture).expect("RR-0535: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0535: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0535: stats visits every byte");
}
