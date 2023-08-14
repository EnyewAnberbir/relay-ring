//! Integration test for `RR-0041` (basic).
//! Ring buffer core export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0041_ring_buffer_core_export_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2c, 0x2e];
    let first = relayring::capabilities::rr_0041_ring_buffer_core_export::evaluate(fixture).expect("RR-0041: Ring buffer core export adapter v16");
    let second = relayring::capabilities::rr_0041_ring_buffer_core_export::evaluate(fixture).expect("RR-0041: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0041: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0041: scanner should emit domain hints");
}
