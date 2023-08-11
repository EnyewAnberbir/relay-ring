//! Integration test for `RR-0027` (basic).
//! Ring buffer core harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0027_ring_buffer_core_harden_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1e, 0x20];
    let first = relayring::capabilities::rr_0027_ring_buffer_core_harden::evaluate(fixture).expect("RR-0027: Ring buffer core harden index v2");
    let second = relayring::capabilities::rr_0027_ring_buffer_core_harden::evaluate(fixture).expect("RR-0027: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0027: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0027: scanner should emit domain hints");
}
