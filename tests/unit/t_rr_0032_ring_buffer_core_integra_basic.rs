//! Integration test for `RR-0032` (basic).
//! Ring buffer core integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0032_ring_buffer_core_integra_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x23, 0x25];
    let first = relayring::capabilities::rr_0032_ring_buffer_core_integra::evaluate(fixture).expect("RR-0032: Ring buffer core integrate validator v7");
    let second = relayring::capabilities::rr_0032_ring_buffer_core_integra::evaluate(fixture).expect("RR-0032: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0032: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0032: stats visits every byte");
}
