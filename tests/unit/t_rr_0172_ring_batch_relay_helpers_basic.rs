//! Integration test for `RR-0172` (basic).
//! Ring batch relay helpers integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0172_ring_batch_relay_helpers_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xaf, 0xb1];
    let first = relayring::capabilities::rr_0172_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0172: Ring batch relay helpers integrate validator v27");
    let second = relayring::capabilities::rr_0172_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0172: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0172: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0172: window consumes the whole buffer");
}
