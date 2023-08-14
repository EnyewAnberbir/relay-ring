//! Integration test for `RR-0040` (basic).
//! Ring buffer core validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0040_ring_buffer_core_validat_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2b, 0x2d];
    let first = relayring::capabilities::rr_0040_ring_buffer_core_validat::evaluate(fixture).expect("RR-0040: Ring buffer core validate resolver v15");
    let second = relayring::capabilities::rr_0040_ring_buffer_core_validat::evaluate(fixture).expect("RR-0040: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0040: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0040: window consumes the whole buffer");
}
