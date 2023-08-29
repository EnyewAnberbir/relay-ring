//! Integration test for `RR-0154` (basic).
//! Ring batch relay helpers benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0154_ring_batch_relay_helpers_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9d, 0x9f];
    let first = relayring::capabilities::rr_0154_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0154: Ring batch relay helpers benchmark reporter v9");
    let second = relayring::capabilities::rr_0154_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0154: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0154: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0154: window consumes the whole buffer");
}
