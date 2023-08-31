//! Integration test for `RR-0170` (basic).
//! Ring batch relay helpers validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0170_ring_batch_relay_helpers_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xad, 0xaf];
    let first = relayring::capabilities::rr_0170_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0170: Ring batch relay helpers validate resolver v25");
    let second = relayring::capabilities::rr_0170_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0170: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0170: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0170: stats visits every byte");
}
