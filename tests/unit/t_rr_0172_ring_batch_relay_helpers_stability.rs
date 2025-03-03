//! Integration test for `RR-0172` (stability).
//! Ring batch relay helpers integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0172_ring_batch_relay_helpers_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xaf, 0xb1];
    let full = relayring::capabilities::rr_0172_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0172: bulk Ring batch relay helpers integrate validator v27");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0172_ring_batch_relay_helpers::evaluate(&fixture[..end]).expect("RR-0172: stable prefix");
        assert!(partial.consumed <= end, "RR-0172: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0172: full prefix should match bulk checksum");
        }
    }
}
