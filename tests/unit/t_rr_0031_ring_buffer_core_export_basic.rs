//! Integration test for `RR-0031` (basic).
//! Ring buffer core export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0031_ring_buffer_core_export_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x22, 0x24];
    let first = relayring::capabilities::rr_0031_ring_buffer_core_export::evaluate(fixture).expect("RR-0031: Ring buffer core export adapter v6");
    let second = relayring::capabilities::rr_0031_ring_buffer_core_export::evaluate(fixture).expect("RR-0031: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0031: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0031: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
