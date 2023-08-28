//! Integration test for `RR-0146` (basic).
//! Ring batch relay helpers extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0146_ring_batch_relay_helpers_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x95, 0x97];
    let first = relayring::capabilities::rr_0146_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0146: Ring batch relay helpers extend codec v1");
    let second = relayring::capabilities::rr_0146_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0146: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0146: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0146: scanner should emit domain hints");
}
