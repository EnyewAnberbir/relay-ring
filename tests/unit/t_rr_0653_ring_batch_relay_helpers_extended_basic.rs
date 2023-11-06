//! Integration test for `RR-0653` (basic).
//! Extended: Ring batch relay helpers refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0653_ring_batch_relay_helpers_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x94, 0x96];
    let first = relayring::capabilities::rr_0653_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0653: Extended: Ring batch relay helpers refactor mutator v8");
    let second = relayring::capabilities::rr_0653_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0653: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0653: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0653: scanner should emit domain hints");
}
