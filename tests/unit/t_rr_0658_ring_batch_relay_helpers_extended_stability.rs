//! Integration test for `RR-0658` (stability).
//! Extended: Ring batch relay helpers wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0658_ring_batch_relay_helpers_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x99, 0x9b];
    let full = relayring::capabilities::rr_0658_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0658: bulk Extended: Ring batch relay helpers wire planner v13");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0658_ring_batch_relay_helpers_extended::evaluate(&fixture[..end]).expect("RR-0658: stable prefix");
        assert!(partial.consumed <= end, "RR-0658: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0658: full prefix should match bulk checksum");
        }
    }
}
