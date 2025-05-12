//! Integration test for `RR-0668` (stability).
//! Extended: Ring batch relay helpers wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0668_ring_batch_relay_helpers_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa3, 0xa5];
    let full = relayring::capabilities::rr_0668_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0668: bulk Extended: Ring batch relay helpers wire planner v23");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0668_ring_batch_relay_helpers_extended::evaluate(&fixture[..end]).expect("RR-0668: stable prefix");
        assert!(partial.consumed <= end, "RR-0668: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0668: full prefix should match bulk checksum");
        }
    }
}
