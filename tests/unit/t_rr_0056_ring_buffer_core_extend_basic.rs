//! Integration test for `RR-0056` (basic).
//! Ring buffer core extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0056_ring_buffer_core_extend_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3b, 0x3d];
    let first = relayring::capabilities::rr_0056_ring_buffer_core_extend::evaluate(fixture).expect("RR-0056: Ring buffer core extend codec v31");
    let second = relayring::capabilities::rr_0056_ring_buffer_core_extend::evaluate(fixture).expect("RR-0056: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0056: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0056: scanner should emit domain hints");
}
