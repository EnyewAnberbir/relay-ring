//! Integration test for `RR-0530` (basic).
//! Extended: Ring buffer core validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0530_ring_buffer_core_validat_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x1b];
    let first = relayring::capabilities::rr_0530_ring_buffer_core_validat_extended::evaluate(fixture).expect("RR-0530: Extended: Ring buffer core validate resolver v5");
    let second = relayring::capabilities::rr_0530_ring_buffer_core_validat_extended::evaluate(fixture).expect("RR-0530: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0530: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0530: scanner should emit domain hints");
}
