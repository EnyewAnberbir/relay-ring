//! Integration test for `RR-0050` (basic).
//! Ring buffer core validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0050_ring_buffer_core_validat_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x35, 0x37];
    let first = relayring::capabilities::rr_0050_ring_buffer_core_validat::evaluate(fixture).expect("RR-0050: Ring buffer core validate resolver v25");
    let second = relayring::capabilities::rr_0050_ring_buffer_core_validat::evaluate(fixture).expect("RR-0050: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0050: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0050: window consumes the whole buffer");
}
