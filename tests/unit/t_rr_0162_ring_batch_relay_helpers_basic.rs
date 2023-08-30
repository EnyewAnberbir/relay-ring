//! Integration test for `RR-0162` (basic).
//! Ring batch relay helpers integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0162_ring_batch_relay_helpers_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa5, 0xa7];
    let first = relayring::capabilities::rr_0162_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0162: Ring batch relay helpers integrate validator v17");
    let second = relayring::capabilities::rr_0162_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0162: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0162: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0162: window consumes the whole buffer");
}
