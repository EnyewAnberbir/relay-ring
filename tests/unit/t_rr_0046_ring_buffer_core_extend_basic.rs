//! Integration test for `RR-0046` (basic).
//! Ring buffer core extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0046_ring_buffer_core_extend_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x31, 0x33];
    let first = relayring::capabilities::rr_0046_ring_buffer_core_extend::evaluate(fixture).expect("RR-0046: Ring buffer core extend codec v21");
    let second = relayring::capabilities::rr_0046_ring_buffer_core_extend::evaluate(fixture).expect("RR-0046: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0046: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0046: scanner should emit domain hints");
}
