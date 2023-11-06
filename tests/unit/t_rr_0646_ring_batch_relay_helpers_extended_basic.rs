//! Integration test for `RR-0646` (basic).
//! Extended: Ring batch relay helpers extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0646_ring_batch_relay_helpers_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8d, 0x8f];
    let first = relayring::capabilities::rr_0646_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0646: Extended: Ring batch relay helpers extend codec v1");
    let second = relayring::capabilities::rr_0646_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0646: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0646: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0646: scanner should emit domain hints");
}
