//! Integration test for `RR-0673` (stability).
//! Extended: Ring batch relay helpers refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0673_ring_batch_relay_helpers_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa8, 0xaa];
    let full = relayring::capabilities::rr_0673_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0673: bulk Extended: Ring batch relay helpers refactor mutator v28");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0673_ring_batch_relay_helpers_extended::evaluate(&fixture[..end]).expect("RR-0673: stable prefix");
        assert!(partial.consumed <= end, "RR-0673: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0673: full prefix should match bulk checksum");
        }
    }
}
