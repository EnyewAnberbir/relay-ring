//! Integration test for `RR-0159` (basic).
//! Ring batch relay helpers optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0159_ring_batch_relay_helpers_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa2, 0xa4];
    let first = relayring::capabilities::rr_0159_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0159: Ring batch relay helpers optimize registry v14");
    let second = relayring::capabilities::rr_0159_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0159: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0159: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0159: scanner should emit domain hints");
}
