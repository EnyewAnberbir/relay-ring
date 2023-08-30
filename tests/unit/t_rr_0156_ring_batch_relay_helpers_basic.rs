//! Integration test for `RR-0156` (basic).
//! Ring batch relay helpers extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0156_ring_batch_relay_helpers_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9f, 0xa1];
    let first = relayring::capabilities::rr_0156_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0156: Ring batch relay helpers extend codec v11");
    let second = relayring::capabilities::rr_0156_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0156: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0156: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0156: scanner should emit domain hints");
}
