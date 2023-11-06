//! Integration test for `RR-0650` (basic).
//! Extended: Ring batch relay helpers validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0650_ring_batch_relay_helpers_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x91, 0x93];
    let first = relayring::capabilities::rr_0650_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0650: Extended: Ring batch relay helpers validate resolver v5");
    let second = relayring::capabilities::rr_0650_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0650: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0650: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0650: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
