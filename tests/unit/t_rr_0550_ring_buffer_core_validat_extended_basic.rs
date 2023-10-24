//! Integration test for `RR-0550` (basic).
//! Extended: Ring buffer core validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0550_ring_buffer_core_validat_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2d, 0x2f];
    let first = relayring::capabilities::rr_0550_ring_buffer_core_validat_extended::evaluate(fixture).expect("RR-0550: Extended: Ring buffer core validate resolver v25");
    let second = relayring::capabilities::rr_0550_ring_buffer_core_validat_extended::evaluate(fixture).expect("RR-0550: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0550: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0550: scanner should emit domain hints");
}
