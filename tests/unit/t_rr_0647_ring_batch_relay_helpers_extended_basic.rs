//! Integration test for `RR-0647` (basic).
//! Extended: Ring batch relay helpers harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0647_ring_batch_relay_helpers_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8e, 0x90];
    let first = relayring::capabilities::rr_0647_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0647: Extended: Ring batch relay helpers harden index v2");
    let second = relayring::capabilities::rr_0647_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0647: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0647: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0647: window consumes the whole buffer");
}
