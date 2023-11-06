//! Integration test for `RR-0655` (basic).
//! Extended: Ring batch relay helpers implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0655_ring_batch_relay_helpers_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x96, 0x98];
    let first = relayring::capabilities::rr_0655_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0655: Extended: Ring batch relay helpers implement pipeline v10");
    let second = relayring::capabilities::rr_0655_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0655: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0655: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0655: window consumes the whole buffer");
}
