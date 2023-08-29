//! Integration test for `RR-0150` (basic).
//! Ring batch relay helpers validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0150_ring_batch_relay_helpers_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x99, 0x9b];
    let first = relayring::capabilities::rr_0150_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0150: Ring batch relay helpers validate resolver v5");
    let second = relayring::capabilities::rr_0150_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0150: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0150: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0150: stats visits every byte");
}
