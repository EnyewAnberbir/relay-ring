//! Integration test for `RR-0169` (basic).
//! Ring batch relay helpers optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0169_ring_batch_relay_helpers_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xac, 0xae];
    let first = relayring::capabilities::rr_0169_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0169: Ring batch relay helpers optimize registry v24");
    let second = relayring::capabilities::rr_0169_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0169: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0169: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0169: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
