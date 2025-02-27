//! Integration test for `RR-0153` (stability).
//! Ring batch relay helpers refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0153_ring_batch_relay_helpers_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9c, 0x9e];
    let full = relayring::capabilities::rr_0153_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0153: bulk Ring batch relay helpers refactor mutator v8");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0153_ring_batch_relay_helpers::evaluate(&fixture[..end]).expect("RR-0153: stable prefix");
        assert!(partial.consumed <= end, "RR-0153: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0153: full prefix should match bulk checksum");
        }
    }
}
