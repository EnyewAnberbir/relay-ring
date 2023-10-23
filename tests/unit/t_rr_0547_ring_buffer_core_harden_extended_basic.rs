//! Integration test for `RR-0547` (basic).
//! Extended: Ring buffer core harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0547_ring_buffer_core_harden_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2a, 0x2c];
    let first = relayring::capabilities::rr_0547_ring_buffer_core_harden_extended::evaluate(fixture).expect("RR-0547: Extended: Ring buffer core harden index v22");
    let second = relayring::capabilities::rr_0547_ring_buffer_core_harden_extended::evaluate(fixture).expect("RR-0547: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0547: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0547: scanner should emit domain hints");
}
