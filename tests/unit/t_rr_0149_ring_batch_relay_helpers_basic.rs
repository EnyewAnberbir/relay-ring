//! Integration test for `RR-0149` (basic).
//! Ring batch relay helpers optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0149_ring_batch_relay_helpers_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x98, 0x9a];
    let first = relayring::capabilities::rr_0149_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0149: Ring batch relay helpers optimize registry v4");
    let second = relayring::capabilities::rr_0149_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0149: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0149: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0149: window consumes the whole buffer");
}
