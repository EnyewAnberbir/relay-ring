//! Integration test for `RR-0171` (basic).
//! Ring batch relay helpers export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0171_ring_batch_relay_helpers_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xae, 0xb0];
    let first = relayring::capabilities::rr_0171_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0171: Ring batch relay helpers export adapter v26");
    let second = relayring::capabilities::rr_0171_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0171: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0171: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0171: window consumes the whole buffer");
}
