//! Integration test for `RR-0039` (basic).
//! Ring buffer core optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0039_ring_buffer_core_optimiz_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2a, 0x2c];
    let first = relayring::capabilities::rr_0039_ring_buffer_core_optimiz::evaluate(fixture).expect("RR-0039: Ring buffer core optimize registry v14");
    let second = relayring::capabilities::rr_0039_ring_buffer_core_optimiz::evaluate(fixture).expect("RR-0039: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0039: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0039: scanner should emit domain hints");
}
