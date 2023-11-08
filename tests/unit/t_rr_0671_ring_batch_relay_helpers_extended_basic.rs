//! Integration test for `RR-0671` (basic).
//! Extended: Ring batch relay helpers export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0671_ring_batch_relay_helpers_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa6, 0xa8];
    let first = relayring::capabilities::rr_0671_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0671: Extended: Ring batch relay helpers export adapter v26");
    let second = relayring::capabilities::rr_0671_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0671: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0671: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0671: scanner should emit domain hints");
}
