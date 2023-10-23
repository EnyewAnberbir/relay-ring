//! Integration test for `RR-0539` (basic).
//! Extended: Ring buffer core optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0539_ring_buffer_core_optimiz_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x22, 0x24];
    let first = relayring::capabilities::rr_0539_ring_buffer_core_optimiz_extended::evaluate(fixture).expect("RR-0539: Extended: Ring buffer core optimize registry v14");
    let second = relayring::capabilities::rr_0539_ring_buffer_core_optimiz_extended::evaluate(fixture).expect("RR-0539: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0539: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0539: scanner should emit domain hints");
}
