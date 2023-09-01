//! Integration test for `RR-0174` (basic).
//! Ring batch relay helpers benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0174_ring_batch_relay_helpers_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb1, 0xb3];
    let first = relayring::capabilities::rr_0174_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0174: Ring batch relay helpers benchmark reporter v29");
    let second = relayring::capabilities::rr_0174_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0174: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0174: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0174: stats visits every byte");
}
