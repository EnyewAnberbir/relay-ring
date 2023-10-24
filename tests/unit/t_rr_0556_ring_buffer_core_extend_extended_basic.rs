//! Integration test for `RR-0556` (basic).
//! Extended: Ring buffer core extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0556_ring_buffer_core_extend_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x33, 0x35];
    let first = relayring::capabilities::rr_0556_ring_buffer_core_extend_extended::evaluate(fixture).expect("RR-0556: Extended: Ring buffer core extend codec v31");
    let second = relayring::capabilities::rr_0556_ring_buffer_core_extend_extended::evaluate(fixture).expect("RR-0556: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0556: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0556: scanner should emit domain hints");
}
