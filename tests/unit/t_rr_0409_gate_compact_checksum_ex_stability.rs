//! Integration test for `RR-0409` (stability).
//! Gate compact checksum export optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0409_gate_compact_checksum_ex_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9e, 0xa0];
    let full = relayring::capabilities::rr_0409_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0409: bulk Gate compact checksum export optimize registry v14");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0409_gate_compact_checksum_ex::evaluate(&fixture[..end]).expect("RR-0409: stable prefix");
        assert!(partial.consumed <= end, "RR-0409: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0409: full prefix should match bulk checksum");
        }
    }
}
