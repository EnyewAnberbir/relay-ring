//! Integration test for `RR-0161` (basic).
//! Ring batch relay helpers export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0161_ring_batch_relay_helpers_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa4, 0xa6];
    let first = relayring::capabilities::rr_0161_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0161: Ring batch relay helpers export adapter v16");
    let second = relayring::capabilities::rr_0161_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0161: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0161: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0161: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
