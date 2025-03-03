//! Integration test for `RR-0168` (stability).
//! Ring batch relay helpers wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0168_ring_batch_relay_helpers_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xab, 0xad];
    let full = relayring::capabilities::rr_0168_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0168: bulk Ring batch relay helpers wire planner v23");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0168_ring_batch_relay_helpers::evaluate(&fixture[..end]).expect("RR-0168: stable prefix");
        assert!(partial.consumed <= end, "RR-0168: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0168: full prefix should match bulk checksum");
        }
    }
}
