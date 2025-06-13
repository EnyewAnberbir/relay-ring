//! Integration test for `RR-0909` (stability).
//! Extended: Gate compact checksum export optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0909_gate_compact_checksum_ex_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x96, 0x98];
    let full = relayring::capabilities::rr_0909_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0909: bulk Extended: Gate compact checksum export optimize registry v14");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0909_gate_compact_checksum_ex_extended::evaluate(&fixture[..end]).expect("RR-0909: stable prefix");
        assert!(partial.consumed <= end, "RR-0909: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0909: full prefix should match bulk checksum");
        }
    }
}
