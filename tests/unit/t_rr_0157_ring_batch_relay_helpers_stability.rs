//! Integration test for `RR-0157` (stability).
//! Ring batch relay helpers harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0157_ring_batch_relay_helpers_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa0, 0xa2];
    let full = relayring::capabilities::rr_0157_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0157: bulk Ring batch relay helpers harden index v12");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0157_ring_batch_relay_helpers::evaluate(&fixture[..end]).expect("RR-0157: stable prefix");
        assert!(partial.consumed <= end, "RR-0157: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0157: full prefix should match bulk checksum");
        }
    }
}
