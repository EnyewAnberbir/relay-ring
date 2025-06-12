//! Integration test for `RR-0898` (stability).
//! Extended: Gate compact checksum export wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0898_gate_compact_checksum_ex_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8b, 0x8d];
    let full = relayring::capabilities::rr_0898_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0898: bulk Extended: Gate compact checksum export wire planner v3");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0898_gate_compact_checksum_ex_extended::evaluate(&fixture[..end]).expect("RR-0898: stable prefix");
        assert!(partial.consumed <= end, "RR-0898: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0898: full prefix should match bulk checksum");
        }
    }
}
