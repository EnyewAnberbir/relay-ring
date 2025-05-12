//! Integration test for `RR-0666` (stability).
//! Extended: Ring batch relay helpers extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0666_ring_batch_relay_helpers_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa1, 0xa3];
    let full = relayring::capabilities::rr_0666_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0666: bulk Extended: Ring batch relay helpers extend codec v21");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0666_ring_batch_relay_helpers_extended::evaluate(&fixture[..end]).expect("RR-0666: stable prefix");
        assert!(partial.consumed <= end, "RR-0666: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0666: full prefix should match bulk checksum");
        }
    }
}
