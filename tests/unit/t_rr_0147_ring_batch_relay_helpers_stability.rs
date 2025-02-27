//! Integration test for `RR-0147` (stability).
//! Ring batch relay helpers harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0147_ring_batch_relay_helpers_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x96, 0x98];
    let full = relayring::capabilities::rr_0147_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0147: bulk Ring batch relay helpers harden index v2");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0147_ring_batch_relay_helpers::evaluate(&fixture[..end]).expect("RR-0147: stable prefix");
        assert!(partial.consumed <= end, "RR-0147: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0147: full prefix should match bulk checksum");
        }
    }
}
