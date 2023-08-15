//! Integration test for `RR-0055` (basic).
//! Ring buffer core implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0055_ring_buffer_core_impleme_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3a, 0x3c];
    let first = relayring::capabilities::rr_0055_ring_buffer_core_impleme::evaluate(fixture).expect("RR-0055: Ring buffer core implement pipeline v30");
    let second = relayring::capabilities::rr_0055_ring_buffer_core_impleme::evaluate(fixture).expect("RR-0055: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0055: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0055: scanner should emit domain hints");
}
