//! Integration test for `RR-0560` (basic).
//! Extended: Ring buffer core validate resolver v35 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0560_ring_buffer_core_validat_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x37, 0x39];
    let first = relayring::capabilities::rr_0560_ring_buffer_core_validat_extended::evaluate(fixture).expect("RR-0560: Extended: Ring buffer core validate resolver v35");
    let second = relayring::capabilities::rr_0560_ring_buffer_core_validat_extended::evaluate(fixture).expect("RR-0560: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0560: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0560: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
