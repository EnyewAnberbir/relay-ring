//! Integration test for `RR-0153` (basic).
//! Ring batch relay helpers refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0153_ring_batch_relay_helpers_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9c, 0x9e];
    let first = relayring::capabilities::rr_0153_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0153: Ring batch relay helpers refactor mutator v8");
    let second = relayring::capabilities::rr_0153_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0153: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0153: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0153: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
