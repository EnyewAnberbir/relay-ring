//! Integration test for `RR-0649` (basic).
//! Extended: Ring batch relay helpers optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0649_ring_batch_relay_helpers_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x90, 0x92];
    let first = relayring::capabilities::rr_0649_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0649: Extended: Ring batch relay helpers optimize registry v4");
    let second = relayring::capabilities::rr_0649_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0649: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0649: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0649: scanner should emit domain hints");
}
