//! Integration test for `RR-0152` (stability).
//! Ring batch relay helpers integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0152_ring_batch_relay_helpers_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9b, 0x9d];
    let full = relayring::capabilities::rr_0152_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0152: bulk Ring batch relay helpers integrate validator v7");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0152_ring_batch_relay_helpers::evaluate(&fixture[..end]).expect("RR-0152: stable prefix");
        assert!(partial.consumed <= end, "RR-0152: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0152: full prefix should match bulk checksum");
        }
    }
}
