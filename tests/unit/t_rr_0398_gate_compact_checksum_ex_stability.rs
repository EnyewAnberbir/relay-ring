//! Integration test for `RR-0398` (stability).
//! Gate compact checksum export wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0398_gate_compact_checksum_ex_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x93, 0x95];
    let full = relayring::capabilities::rr_0398_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0398: bulk Gate compact checksum export wire planner v3");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0398_gate_compact_checksum_ex::evaluate(&fixture[..end]).expect("RR-0398: stable prefix");
        assert!(partial.consumed <= end, "RR-0398: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0398: full prefix should match bulk checksum");
        }
    }
}
