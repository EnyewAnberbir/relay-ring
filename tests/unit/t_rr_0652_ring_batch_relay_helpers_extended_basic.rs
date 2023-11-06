//! Integration test for `RR-0652` (basic).
//! Extended: Ring batch relay helpers integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0652_ring_batch_relay_helpers_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x93, 0x95];
    let first = relayring::capabilities::rr_0652_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0652: Extended: Ring batch relay helpers integrate validator v7");
    let second = relayring::capabilities::rr_0652_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0652: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0652: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0652: stats visits every byte");
}
