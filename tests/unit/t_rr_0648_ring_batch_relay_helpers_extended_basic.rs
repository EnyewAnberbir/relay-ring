//! Integration test for `RR-0648` (basic).
//! Extended: Ring batch relay helpers wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0648_ring_batch_relay_helpers_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8f, 0x91];
    let first = relayring::capabilities::rr_0648_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0648: Extended: Ring batch relay helpers wire planner v3");
    let second = relayring::capabilities::rr_0648_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0648: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0648: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0648: stats visits every byte");
}
