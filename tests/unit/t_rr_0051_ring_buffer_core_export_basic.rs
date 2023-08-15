//! Integration test for `RR-0051` (basic).
//! Ring buffer core export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0051_ring_buffer_core_export_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x36, 0x38];
    let first = relayring::capabilities::rr_0051_ring_buffer_core_export::evaluate(fixture).expect("RR-0051: Ring buffer core export adapter v26");
    let second = relayring::capabilities::rr_0051_ring_buffer_core_export::evaluate(fixture).expect("RR-0051: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0051: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0051: scanner should emit domain hints");
}
