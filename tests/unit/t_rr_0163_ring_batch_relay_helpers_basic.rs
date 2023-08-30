//! Integration test for `RR-0163` (basic).
//! Ring batch relay helpers refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0163_ring_batch_relay_helpers_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa6, 0xa8];
    let first = relayring::capabilities::rr_0163_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0163: Ring batch relay helpers refactor mutator v18");
    let second = relayring::capabilities::rr_0163_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0163: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0163: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0163: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
