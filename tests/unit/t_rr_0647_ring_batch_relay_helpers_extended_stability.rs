//! Integration test for `RR-0647` (stability).
//! Extended: Ring batch relay helpers harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0647_ring_batch_relay_helpers_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8e, 0x90];
    let full = relayring::capabilities::rr_0647_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0647: bulk Extended: Ring batch relay helpers harden index v2");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0647_ring_batch_relay_helpers_extended::evaluate(&fixture[..end]).expect("RR-0647: stable prefix");
        assert!(partial.consumed <= end, "RR-0647: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0647: full prefix should match bulk checksum");
        }
    }
}
