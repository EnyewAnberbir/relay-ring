//! Integration test for `RR-0151` (basic).
//! Ring batch relay helpers export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0151_ring_batch_relay_helpers_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9a, 0x9c];
    let first = relayring::capabilities::rr_0151_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0151: Ring batch relay helpers export adapter v6");
    let second = relayring::capabilities::rr_0151_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0151: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0151: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0151: stats visits every byte");
}
