//! Integration test for `RR-0551` (basic).
//! Extended: Ring buffer core export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0551_ring_buffer_core_export_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2e, 0x30];
    let first = relayring::capabilities::rr_0551_ring_buffer_core_export_extended::evaluate(fixture).expect("RR-0551: Extended: Ring buffer core export adapter v26");
    let second = relayring::capabilities::rr_0551_ring_buffer_core_export_extended::evaluate(fixture).expect("RR-0551: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0551: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0551: scanner should emit domain hints");
}
