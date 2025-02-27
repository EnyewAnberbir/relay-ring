//! Integration test for `RR-0151` (stability).
//! Ring batch relay helpers export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0151_ring_batch_relay_helpers_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9a, 0x9c];
    let full = relayring::capabilities::rr_0151_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0151: bulk Ring batch relay helpers export adapter v6");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0151_ring_batch_relay_helpers::evaluate(&fixture[..end]).expect("RR-0151: stable prefix");
        assert!(partial.consumed <= end, "RR-0151: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0151: full prefix should match bulk checksum");
        }
    }
}
