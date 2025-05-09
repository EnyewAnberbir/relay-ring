//! Integration test for `RR-0659` (stability).
//! Extended: Ring batch relay helpers optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0659_ring_batch_relay_helpers_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9a, 0x9c];
    let full = relayring::capabilities::rr_0659_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0659: bulk Extended: Ring batch relay helpers optimize registry v14");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0659_ring_batch_relay_helpers_extended::evaluate(&fixture[..end]).expect("RR-0659: stable prefix");
        assert!(partial.consumed <= end, "RR-0659: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0659: full prefix should match bulk checksum");
        }
    }
}
