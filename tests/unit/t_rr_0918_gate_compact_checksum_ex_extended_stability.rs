//! Integration test for `RR-0918` (stability).
//! Extended: Gate compact checksum export wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0918_gate_compact_checksum_ex_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9f, 0xa1];
    let full = relayring::capabilities::rr_0918_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0918: bulk Extended: Gate compact checksum export wire planner v23");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0918_gate_compact_checksum_ex_extended::evaluate(&fixture[..end]).expect("RR-0918: stable prefix");
        assert!(partial.consumed <= end, "RR-0918: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0918: full prefix should match bulk checksum");
        }
    }
}
