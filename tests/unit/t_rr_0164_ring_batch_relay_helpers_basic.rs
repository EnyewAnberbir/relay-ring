//! Integration test for `RR-0164` (basic).
//! Ring batch relay helpers benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0164_ring_batch_relay_helpers_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa7, 0xa9];
    let first = relayring::capabilities::rr_0164_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0164: Ring batch relay helpers benchmark reporter v19");
    let second = relayring::capabilities::rr_0164_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0164: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0164: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0164: scanner should emit domain hints");
}
