//! Integration test for `RR-0174` (stability).
//! Ring batch relay helpers benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0174_ring_batch_relay_helpers_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb1, 0xb3];
    let full = relayring::capabilities::rr_0174_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0174: bulk Ring batch relay helpers benchmark reporter v29");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0174_ring_batch_relay_helpers::evaluate(&fixture[..end]).expect("RR-0174: stable prefix");
        assert!(partial.consumed <= end, "RR-0174: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0174: full prefix should match bulk checksum");
        }
    }
}
