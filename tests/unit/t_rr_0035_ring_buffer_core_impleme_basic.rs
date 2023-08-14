//! Integration test for `RR-0035` (basic).
//! Ring buffer core implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0035_ring_buffer_core_impleme_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x26, 0x28];
    let first = relayring::capabilities::rr_0035_ring_buffer_core_impleme::evaluate(fixture).expect("RR-0035: Ring buffer core implement pipeline v10");
    let second = relayring::capabilities::rr_0035_ring_buffer_core_impleme::evaluate(fixture).expect("RR-0035: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0035: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0035: window consumes the whole buffer");
}
