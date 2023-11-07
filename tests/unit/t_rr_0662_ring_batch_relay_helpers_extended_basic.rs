//! Integration test for `RR-0662` (basic).
//! Extended: Ring batch relay helpers integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0662_ring_batch_relay_helpers_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9d, 0x9f];
    let first = relayring::capabilities::rr_0662_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0662: Extended: Ring batch relay helpers integrate validator v17");
    let second = relayring::capabilities::rr_0662_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0662: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0662: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0662: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
