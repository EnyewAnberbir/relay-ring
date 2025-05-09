//! Integration test for `RR-0656` (stability).
//! Extended: Ring batch relay helpers extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0656_ring_batch_relay_helpers_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x97, 0x99];
    let full = relayring::capabilities::rr_0656_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0656: bulk Extended: Ring batch relay helpers extend codec v11");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0656_ring_batch_relay_helpers_extended::evaluate(&fixture[..end]).expect("RR-0656: stable prefix");
        assert!(partial.consumed <= end, "RR-0656: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0656: full prefix should match bulk checksum");
        }
    }
}
