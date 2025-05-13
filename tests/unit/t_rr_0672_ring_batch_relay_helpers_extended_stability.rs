//! Integration test for `RR-0672` (stability).
//! Extended: Ring batch relay helpers integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0672_ring_batch_relay_helpers_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa7, 0xa9];
    let full = relayring::capabilities::rr_0672_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0672: bulk Extended: Ring batch relay helpers integrate validator v27");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0672_ring_batch_relay_helpers_extended::evaluate(&fixture[..end]).expect("RR-0672: stable prefix");
        assert!(partial.consumed <= end, "RR-0672: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0672: full prefix should match bulk checksum");
        }
    }
}
