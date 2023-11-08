//! Integration test for `RR-0674` (basic).
//! Extended: Ring batch relay helpers benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0674_ring_batch_relay_helpers_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa9, 0xab];
    let first = relayring::capabilities::rr_0674_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0674: Extended: Ring batch relay helpers benchmark reporter v29");
    let second = relayring::capabilities::rr_0674_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0674: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0674: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0674: stats visits every byte");
}
