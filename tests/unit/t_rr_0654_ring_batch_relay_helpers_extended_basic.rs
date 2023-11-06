//! Integration test for `RR-0654` (basic).
//! Extended: Ring batch relay helpers benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0654_ring_batch_relay_helpers_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x95, 0x97];
    let first = relayring::capabilities::rr_0654_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0654: Extended: Ring batch relay helpers benchmark reporter v9");
    let second = relayring::capabilities::rr_0654_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0654: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0654: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0654: stats visits every byte");
}
