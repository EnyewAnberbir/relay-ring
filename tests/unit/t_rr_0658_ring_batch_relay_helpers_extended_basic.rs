//! Integration test for `RR-0658` (basic).
//! Extended: Ring batch relay helpers wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0658_ring_batch_relay_helpers_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x99, 0x9b];
    let first = relayring::capabilities::rr_0658_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0658: Extended: Ring batch relay helpers wire planner v13");
    let second = relayring::capabilities::rr_0658_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0658: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0658: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0658: scanner should emit domain hints");
}
