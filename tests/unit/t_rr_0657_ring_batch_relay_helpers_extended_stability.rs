//! Integration test for `RR-0657` (stability).
//! Extended: Ring batch relay helpers harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0657_ring_batch_relay_helpers_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x98, 0x9a];
    let full = relayring::capabilities::rr_0657_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0657: bulk Extended: Ring batch relay helpers harden index v12");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0657_ring_batch_relay_helpers_extended::evaluate(&fixture[..end]).expect("RR-0657: stable prefix");
        assert!(partial.consumed <= end, "RR-0657: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0657: full prefix should match bulk checksum");
        }
    }
}
