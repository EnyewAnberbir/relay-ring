//! Integration test for `RR-0660` (stability).
//! Extended: Ring batch relay helpers validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0660_ring_batch_relay_helpers_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9b, 0x9d];
    let full = relayring::capabilities::rr_0660_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0660: bulk Extended: Ring batch relay helpers validate resolver v15");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0660_ring_batch_relay_helpers_extended::evaluate(&fixture[..end]).expect("RR-0660: stable prefix");
        assert!(partial.consumed <= end, "RR-0660: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0660: full prefix should match bulk checksum");
        }
    }
}
