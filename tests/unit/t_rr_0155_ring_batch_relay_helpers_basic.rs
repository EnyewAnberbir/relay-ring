//! Integration test for `RR-0155` (basic).
//! Ring batch relay helpers implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0155_ring_batch_relay_helpers_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9e, 0xa0];
    let first = relayring::capabilities::rr_0155_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0155: Ring batch relay helpers implement pipeline v10");
    let second = relayring::capabilities::rr_0155_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0155: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0155: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0155: window consumes the whole buffer");
}
