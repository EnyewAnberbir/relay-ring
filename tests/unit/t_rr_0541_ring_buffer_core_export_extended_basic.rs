//! Integration test for `RR-0541` (basic).
//! Extended: Ring buffer core export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0541_ring_buffer_core_export_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x24, 0x26];
    let first = relayring::capabilities::rr_0541_ring_buffer_core_export_extended::evaluate(fixture).expect("RR-0541: Extended: Ring buffer core export adapter v16");
    let second = relayring::capabilities::rr_0541_ring_buffer_core_export_extended::evaluate(fixture).expect("RR-0541: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0541: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0541: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
