//! Integration test for `RR-0665` (basic).
//! Extended: Ring batch relay helpers implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0665_ring_batch_relay_helpers_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa0, 0xa2];
    let first = relayring::capabilities::rr_0665_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0665: Extended: Ring batch relay helpers implement pipeline v20");
    let second = relayring::capabilities::rr_0665_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0665: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0665: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0665: window consumes the whole buffer");
}
