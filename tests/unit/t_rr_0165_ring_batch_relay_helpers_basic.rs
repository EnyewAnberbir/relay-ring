//! Integration test for `RR-0165` (basic).
//! Ring batch relay helpers implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0165_ring_batch_relay_helpers_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa8, 0xaa];
    let first = relayring::capabilities::rr_0165_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0165: Ring batch relay helpers implement pipeline v20");
    let second = relayring::capabilities::rr_0165_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0165: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0165: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0165: stats visits every byte");
}
