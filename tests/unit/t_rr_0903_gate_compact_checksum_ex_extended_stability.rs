//! Integration test for `RR-0903` (stability).
//! Extended: Gate compact checksum export refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0903_gate_compact_checksum_ex_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x90, 0x92];
    let full = relayring::capabilities::rr_0903_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0903: bulk Extended: Gate compact checksum export refactor mutator v8");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0903_gate_compact_checksum_ex_extended::evaluate(&fixture[..end]).expect("RR-0903: stable prefix");
        assert!(partial.consumed <= end, "RR-0903: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0903: full prefix should match bulk checksum");
        }
    }
}
