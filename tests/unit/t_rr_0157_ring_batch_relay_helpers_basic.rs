//! Integration test for `RR-0157` (basic).
//! Ring batch relay helpers harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0157_ring_batch_relay_helpers_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa0, 0xa2];
    let first = relayring::capabilities::rr_0157_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0157: Ring batch relay helpers harden index v12");
    let second = relayring::capabilities::rr_0157_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0157: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0157: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0157: stats visits every byte");
}
