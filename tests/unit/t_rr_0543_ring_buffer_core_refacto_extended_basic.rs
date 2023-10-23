//! Integration test for `RR-0543` (basic).
//! Extended: Ring buffer core refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0543_ring_buffer_core_refacto_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x26, 0x28];
    let first = relayring::capabilities::rr_0543_ring_buffer_core_refacto_extended::evaluate(fixture).expect("RR-0543: Extended: Ring buffer core refactor mutator v18");
    let second = relayring::capabilities::rr_0543_ring_buffer_core_refacto_extended::evaluate(fixture).expect("RR-0543: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0543: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0543: stats visits every byte");
}
