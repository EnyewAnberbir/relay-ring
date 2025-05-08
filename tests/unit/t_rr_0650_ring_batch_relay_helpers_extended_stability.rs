//! Integration test for `RR-0650` (stability).
//! Extended: Ring batch relay helpers validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0650_ring_batch_relay_helpers_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x91, 0x93];
    let full = relayring::capabilities::rr_0650_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0650: bulk Extended: Ring batch relay helpers validate resolver v5");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0650_ring_batch_relay_helpers_extended::evaluate(&fixture[..end]).expect("RR-0650: stable prefix");
        assert!(partial.consumed <= end, "RR-0650: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0650: full prefix should match bulk checksum");
        }
    }
}
