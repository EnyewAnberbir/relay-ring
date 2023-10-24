//! Integration test for `RR-0549` (basic).
//! Extended: Ring buffer core optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0549_ring_buffer_core_optimiz_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2c, 0x2e];
    let first = relayring::capabilities::rr_0549_ring_buffer_core_optimiz_extended::evaluate(fixture).expect("RR-0549: Extended: Ring buffer core optimize registry v24");
    let second = relayring::capabilities::rr_0549_ring_buffer_core_optimiz_extended::evaluate(fixture).expect("RR-0549: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0549: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0549: scanner should emit domain hints");
}
