//! Integration test for `RR-0168` (basic).
//! Ring batch relay helpers wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0168_ring_batch_relay_helpers_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xab, 0xad];
    let first = relayring::capabilities::rr_0168_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0168: Ring batch relay helpers wire planner v23");
    let second = relayring::capabilities::rr_0168_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0168: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0168: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0168: scanner should emit domain hints");
}
