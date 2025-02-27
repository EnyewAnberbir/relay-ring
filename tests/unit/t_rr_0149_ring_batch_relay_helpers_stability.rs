//! Integration test for `RR-0149` (stability).
//! Ring batch relay helpers optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0149_ring_batch_relay_helpers_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x98, 0x9a];
    let full = relayring::capabilities::rr_0149_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0149: bulk Ring batch relay helpers optimize registry v4");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0149_ring_batch_relay_helpers::evaluate(&fixture[..end]).expect("RR-0149: stable prefix");
        assert!(partial.consumed <= end, "RR-0149: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0149: full prefix should match bulk checksum");
        }
    }
}
