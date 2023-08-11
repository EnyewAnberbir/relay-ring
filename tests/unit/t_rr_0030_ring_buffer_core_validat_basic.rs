//! Integration test for `RR-0030` (basic).
//! Ring buffer core validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0030_ring_buffer_core_validat_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x21, 0x23];
    let first = relayring::capabilities::rr_0030_ring_buffer_core_validat::evaluate(fixture).expect("RR-0030: Ring buffer core validate resolver v5");
    let second = relayring::capabilities::rr_0030_ring_buffer_core_validat::evaluate(fixture).expect("RR-0030: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0030: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0030: scanner should emit domain hints");
}
