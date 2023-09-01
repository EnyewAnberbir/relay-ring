//! Integration test for `RR-0175` (basic).
//! Ring batch relay helpers implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0175_ring_batch_relay_helpers_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb2, 0xb4];
    let first = relayring::capabilities::rr_0175_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0175: Ring batch relay helpers implement pipeline v30");
    let second = relayring::capabilities::rr_0175_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0175: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0175: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0175: window consumes the whole buffer");
}
