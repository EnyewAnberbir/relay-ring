//! Integration test for `RR-0160` (basic).
//! Ring batch relay helpers validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0160_ring_batch_relay_helpers_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa3, 0xa5];
    let first = relayring::capabilities::rr_0160_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0160: Ring batch relay helpers validate resolver v15");
    let second = relayring::capabilities::rr_0160_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0160: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0160: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0160: stats visits every byte");
}
