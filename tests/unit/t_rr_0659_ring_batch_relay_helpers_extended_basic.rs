//! Integration test for `RR-0659` (basic).
//! Extended: Ring batch relay helpers optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0659_ring_batch_relay_helpers_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9a, 0x9c];
    let first = relayring::capabilities::rr_0659_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0659: Extended: Ring batch relay helpers optimize registry v14");
    let second = relayring::capabilities::rr_0659_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0659: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0659: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0659: stats visits every byte");
}
