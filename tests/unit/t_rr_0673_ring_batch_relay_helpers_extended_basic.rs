//! Integration test for `RR-0673` (basic).
//! Extended: Ring batch relay helpers refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0673_ring_batch_relay_helpers_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa8, 0xaa];
    let first = relayring::capabilities::rr_0673_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0673: Extended: Ring batch relay helpers refactor mutator v28");
    let second = relayring::capabilities::rr_0673_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0673: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0673: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0673: stats visits every byte");
}
