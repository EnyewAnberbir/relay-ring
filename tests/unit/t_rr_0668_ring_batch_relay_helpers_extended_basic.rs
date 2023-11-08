//! Integration test for `RR-0668` (basic).
//! Extended: Ring batch relay helpers wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0668_ring_batch_relay_helpers_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa3, 0xa5];
    let first = relayring::capabilities::rr_0668_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0668: Extended: Ring batch relay helpers wire planner v23");
    let second = relayring::capabilities::rr_0668_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0668: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0668: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0668: window consumes the whole buffer");
}
