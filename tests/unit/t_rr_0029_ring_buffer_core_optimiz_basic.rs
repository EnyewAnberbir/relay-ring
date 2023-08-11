//! Integration test for `RR-0029` (basic).
//! Ring buffer core optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0029_ring_buffer_core_optimiz_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x22];
    let first = relayring::capabilities::rr_0029_ring_buffer_core_optimiz::evaluate(fixture).expect("RR-0029: Ring buffer core optimize registry v4");
    let second = relayring::capabilities::rr_0029_ring_buffer_core_optimiz::evaluate(fixture).expect("RR-0029: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0029: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0029: window consumes the whole buffer");
}
