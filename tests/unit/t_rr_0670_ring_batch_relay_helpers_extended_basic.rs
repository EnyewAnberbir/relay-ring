//! Integration test for `RR-0670` (basic).
//! Extended: Ring batch relay helpers validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0670_ring_batch_relay_helpers_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa5, 0xa7];
    let first = relayring::capabilities::rr_0670_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0670: Extended: Ring batch relay helpers validate resolver v25");
    let second = relayring::capabilities::rr_0670_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0670: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0670: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0670: stats visits every byte");
}
