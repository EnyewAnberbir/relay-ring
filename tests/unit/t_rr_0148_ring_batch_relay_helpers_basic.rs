//! Integration test for `RR-0148` (basic).
//! Ring batch relay helpers wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0148_ring_batch_relay_helpers_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x97, 0x99];
    let first = relayring::capabilities::rr_0148_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0148: Ring batch relay helpers wire planner v3");
    let second = relayring::capabilities::rr_0148_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0148: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0148: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0148: stats visits every byte");
}
