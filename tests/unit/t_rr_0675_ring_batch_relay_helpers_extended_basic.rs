//! Integration test for `RR-0675` (basic).
//! Extended: Ring batch relay helpers implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0675_ring_batch_relay_helpers_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xaa, 0xac];
    let first = relayring::capabilities::rr_0675_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0675: Extended: Ring batch relay helpers implement pipeline v30");
    let second = relayring::capabilities::rr_0675_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0675: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0675: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0675: window consumes the whole buffer");
}
