//! Integration test for `RR-0037` (basic).
//! Ring buffer core harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0037_ring_buffer_core_harden_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x28, 0x2a];
    let first = relayring::capabilities::rr_0037_ring_buffer_core_harden::evaluate(fixture).expect("RR-0037: Ring buffer core harden index v12");
    let second = relayring::capabilities::rr_0037_ring_buffer_core_harden::evaluate(fixture).expect("RR-0037: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0037: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0037: stats visits every byte");
}
