//! Integration test for `RR-0059` (basic).
//! Ring buffer core optimize registry v34 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0059_ring_buffer_core_optimiz_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3e, 0x40];
    let first = relayring::capabilities::rr_0059_ring_buffer_core_optimiz::evaluate(fixture).expect("RR-0059: Ring buffer core optimize registry v34");
    let second = relayring::capabilities::rr_0059_ring_buffer_core_optimiz::evaluate(fixture).expect("RR-0059: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0059: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0059: scanner should emit domain hints");
}
